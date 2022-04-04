//! Functionality for comparing
//! This is mainly used to build the triage report and the perf
//! comparison endpoints

use crate::api;
use crate::db::{ArtifactId, Benchmark, Lookup, Profile, Scenario};
use crate::github;
use crate::load::SiteCtxt;
use crate::selector::{self, Tag};

use collector::category::Category;
use collector::Bound;
use serde::Serialize;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;

type BoxedError = Box<dyn Error + Send + Sync>;

pub async fn handle_triage(
    body: api::triage::Request,
    ctxt: &SiteCtxt,
) -> api::ServerResult<api::triage::Response> {
    log::info!("handle_triage({:?})", body);
    let start = body.start;
    let end = body.end;
    let master_commits = &ctxt.get_master_commits().commits;

    let start_artifact = ctxt
        .artifact_id_for_bound(start.clone(), true)
        .ok_or(format!("could not find start commit for bound {:?}", start))?;
    // This gives a better error, but is still not great -- the common case here
    // is that we've had a 422 error and as such had a fork. It's possible we
    // could diagnose that and give a nicer error here telling the user which
    // commit to use.
    let mut next = next_commit(&start_artifact, &master_commits)
        .map(|c| Bound::Commit(c.sha.clone()))
        .ok_or(format!("no next commit for {:?}", start_artifact))?;

    let mut report = HashMap::new();
    let mut before = start.clone();

    let mut num_comparisons = 0;
    loop {
        let comparison = match compare_given_commits(
            before,
            next.clone(),
            "instructions:u".to_owned(),
            ctxt,
            &master_commits,
        )
        .await
        .map_err(|e| format!("error comparing commits: {}", e))?
        {
            Some(c) => c,
            None => {
                log::info!(
                    "No data found for end bound {:?}. Ending comparison...",
                    next
                );
                break;
            }
        };
        num_comparisons += 1;
        log::info!(
            "Comparing {} to {}",
            comparison.b.artifact,
            comparison.a.artifact
        );

        // handle results of comparison
        populate_report(ctxt, &comparison, &mut report).await;

        // Check that there is a next commit and that the
        // after commit is not equal to `end`
        match comparison.next(&master_commits).map(Bound::Commit) {
            Some(n) if Some(&next) != end.as_ref() => {
                before = next;
                next = n;
            }
            _ => break,
        }
    }
    let end = end.unwrap_or(next);

    let report = generate_report(&start, &end, report, num_comparisons).await;
    Ok(api::triage::Response(report))
}

pub async fn handle_compare(
    body: api::comparison::Request,
    ctxt: &SiteCtxt,
) -> api::ServerResult<api::comparison::Response> {
    log::info!("handle_compare({:?})", body);
    let master_commits = &ctxt.get_master_commits().commits;

    let end = body.end;
    let comparison =
        compare_given_commits(body.start, end.clone(), body.stat, ctxt, master_commits)
            .await
            .map_err(|e| format!("error comparing commits: {}", e))?
            .ok_or_else(|| format!("could not find end commit for bound {:?}", end))?;

    let conn = ctxt.conn().await;
    let prev = comparison.prev(master_commits);
    let next = comparison.next(master_commits);
    let is_contiguous = comparison.is_contiguous(&*conn, master_commits).await;
    let benchmark_map = conn.get_benchmarks().await;

    let comparisons = comparison
        .statistics
        .into_iter()
        .map(|comparison| api::comparison::Comparison {
            benchmark: comparison.benchmark.to_string(),
            profile: comparison.profile.to_string(),
            scenario: comparison.scenario.to_string(),
            is_dodgy: comparison.is_dodgy(),
            is_significant: comparison.is_significant(),
            significance_factor: comparison.significance_factor(),
            magnitude: comparison.magnitude().display().to_owned(),
            statistics: comparison.results,
        })
        .collect();

    let mut new_errors = comparison.new_errors.into_iter().collect::<Vec<_>>();
    new_errors.sort();
    Ok(api::comparison::Response {
        prev,
        a: comparison.a.into(),
        b: comparison.b.into(),
        comparisons,
        new_errors,
        next,
        is_contiguous,
        benchmark_data: benchmark_map
            .into_iter()
            .map(|bench| bench.into())
            .collect(),
    })
}

async fn populate_report(
    ctxt: &SiteCtxt,
    comparison: &Comparison,
    report: &mut HashMap<Option<Direction>, Vec<String>>,
) {
    if let Some(summary) = ComparisonSummary::summarize_comparison(comparison) {
        let confidence = summary.confidence();
        if confidence.is_atleast_probably_relevant() {
            if let Some(direction) = summary.direction() {
                let entry = report
                    .entry(confidence.is_definitely_relevant().then(|| direction))
                    .or_default();

                entry.push(write_triage_summary(ctxt, comparison).await)
            }
        }
    }
}

/// A summary of a given comparison
///
/// This summary only includes changes that are significant and relevant (as determined by a changes magnitude).
pub struct ComparisonSummary {
    /// Significant comparisons of magnitude small and above
    /// and ordered by magnitude from largest to smallest
    comparisons: Vec<TestResultComparison>,
    /// The cached number of comparisons that are improvements
    num_improvements: usize,
    /// The cached number of comparisons that are regressions
    num_regressions: usize,
    /// Which benchmarks had errors
    errors_in: Vec<String>,
}

impl ComparisonSummary {
    pub fn summarize_comparison(comparison: &Comparison) -> Option<Self> {
        let mut num_improvements = 0;
        let mut num_regressions = 0;

        let mut comparisons = comparison
            .statistics
            .iter()
            .filter(|c| c.is_significant())
            .filter(|c| c.magnitude().is_small_or_above())
            .inspect(|c| {
                if c.is_improvement() {
                    num_improvements += 1;
                } else {
                    num_regressions += 1
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        // Skip empty commits, sometimes happens if there's a compiler bug or so.
        if comparisons.len() == 0 {
            return None;
        }

        let cmp = |b1: &TestResultComparison, b2: &TestResultComparison| {
            b2.relative_change()
                .abs()
                .partial_cmp(&b1.relative_change().abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        };
        comparisons.sort_by(cmp);

        let errors_in = comparison.new_errors.keys().cloned().collect::<Vec<_>>();

        Some(ComparisonSummary {
            comparisons,
            num_improvements,
            num_regressions,
            errors_in,
        })
    }

    pub fn empty() -> Self {
        ComparisonSummary {
            comparisons: vec![],
            num_improvements: 0,
            num_regressions: 0,
            errors_in: vec![],
        }
    }

    /// The direction of the changes
    pub fn direction(&self) -> Option<Direction> {
        if self.comparisons.len() == 0 {
            return None;
        }

        let (regressions, improvements): (Vec<&TestResultComparison>, _) =
            self.comparisons.iter().partition(|c| c.is_regression());

        if regressions.len() == 0 {
            return Some(Direction::Improvement);
        }

        if improvements.len() == 0 {
            return Some(Direction::Regression);
        }

        let total_num = self.comparisons.len();
        let regressions_ratio = regressions.len() as f64 / total_num as f64;

        let has_medium_and_above_regressions = regressions
            .iter()
            .any(|c| c.magnitude().is_medium_or_above());
        let has_medium_and_above_improvements = improvements
            .iter()
            .any(|c| c.magnitude().is_medium_or_above());
        match (
            has_medium_and_above_improvements,
            has_medium_and_above_regressions,
        ) {
            (true, true) => return Some(Direction::Mixed),
            (true, false) => {
                if regressions_ratio >= 0.15 {
                    Some(Direction::Mixed)
                } else {
                    Some(Direction::Improvement)
                }
            }
            (false, true) => {
                if regressions_ratio < 0.85 {
                    Some(Direction::Mixed)
                } else {
                    Some(Direction::Regression)
                }
            }
            (false, false) => {
                if regressions_ratio >= 0.1 && regressions_ratio <= 0.9 {
                    Some(Direction::Mixed)
                } else if regressions_ratio <= 0.1 {
                    Some(Direction::Improvement)
                } else {
                    Some(Direction::Regression)
                }
            }
        }
    }

    /// The number of improvements that were found to be significant and relevant
    pub fn number_of_improvements(&self) -> usize {
        self.num_improvements
    }

    /// The number of regressions that were found to be significant and relevant
    pub fn number_of_regressions(&self) -> usize {
        self.num_regressions
    }

    /// Arithmetic mean of all improvements as a percent
    pub fn arithmetic_mean_of_improvements(&self) -> f64 {
        self.arithmetic_mean(self.improvements())
    }

    /// Arithmetic mean of all regressions as a percent
    pub fn arithmetic_mean_of_regressions(&self) -> f64 {
        self.arithmetic_mean(self.regressions())
    }

    /// Arithmetic mean of all changes as a percent
    pub fn arithmetic_mean_of_changes(&self) -> f64 {
        self.arithmetic_mean(self.comparisons.iter())
    }

    pub fn num_significant_changes(&self) -> usize {
        self.comparisons
            .iter()
            .filter(|c| c.is_significant())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.comparisons.is_empty()
    }

    pub fn errors_in(&self) -> &[String] {
        &self.errors_in
    }

    fn arithmetic_mean<'a>(
        &'a self,
        changes: impl Iterator<Item = &'a TestResultComparison>,
    ) -> f64 {
        let mut count = 0;
        let mut sum = 0.0;
        for r in changes {
            sum += r.relative_change();
            count += 1;
        }

        (sum / count as f64) * 100.0
    }

    fn improvements(&self) -> impl Iterator<Item = &TestResultComparison> {
        self.comparisons.iter().filter(|c| c.is_improvement())
    }

    fn regressions(&self) -> impl Iterator<Item = &TestResultComparison> {
        self.comparisons.iter().filter(|c| c.is_regression())
    }

    fn largest_improvement(&self) -> Option<&TestResultComparison> {
        self.comparisons.iter().find(|s| s.is_improvement())
    }

    fn largest_regression(&self) -> Option<&TestResultComparison> {
        self.comparisons.iter().find(|s| s.is_regression())
    }

    pub fn confidence(&self) -> ComparisonConfidence {
        let mut num_small_changes = 0;
        let mut num_medium_changes = 0;
        for c in self.comparisons.iter() {
            match c.magnitude() {
                Magnitude::Small => num_small_changes += 1,
                Magnitude::Medium => num_medium_changes += 1,
                Magnitude::Large => return ComparisonConfidence::DefinitelyRelevant,
                Magnitude::VeryLarge => return ComparisonConfidence::DefinitelyRelevant,
                Magnitude::VerySmall => unreachable!(),
            }
        }

        match (num_small_changes, num_medium_changes) {
            (_, m) if m > 1 => ComparisonConfidence::DefinitelyRelevant,
            (_, 1) => ComparisonConfidence::ProbablyRelevant,
            (s, 0) if s > 10 => ComparisonConfidence::ProbablyRelevant,
            _ => ComparisonConfidence::MaybeRelevant,
        }
    }
}

async fn write_triage_summary(ctxt: &SiteCtxt, comparison: &Comparison) -> String {
    use std::fmt::Write;
    let mut result = if let Some(pr) = comparison.b.pr {
        let title = github::pr_title(pr).await;
        format!(
            "{} [#{}](https://github.com/rust-lang/rust/pull/{})",
            title, pr, pr
        )
    } else {
        String::from("<Unknown Change>")
    };
    let start = &comparison.a.artifact;
    let end = &comparison.b.artifact;
    let link = &compare_link(start, end);
    write!(&mut result, " [(Comparison Link)]({})\n", link).unwrap();

    let benchmark_map = ctxt.get_benchmark_category_map().await;
    let (primary, secondary) = comparison.clone().summarize_by_category(benchmark_map);
    let primary = primary.unwrap_or_else(ComparisonSummary::empty);
    let secondary = secondary.unwrap_or_else(ComparisonSummary::empty);

    write_summary_table(&primary, &secondary, false, &mut result);

    result
}

/// Writes a Markdown table containing summary of relevant results.
pub fn write_summary_table(
    primary: &ComparisonSummary,
    secondary: &ComparisonSummary,
    with_footnotes: bool,
    result: &mut String,
) {
    use std::fmt::Write;

    fn render_stat<F: FnOnce() -> Option<f64>>(count: usize, calculate: F) -> String {
        let value = if count > 0 { calculate() } else { None };
        value
            .map(|value| format!("{value:.1}%"))
            .unwrap_or_else(|| "N/A".to_string())
    }

    writeln!(
        result,
        r#"| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|"#
    )
    .unwrap();
    writeln!(
        result,
        "| count{} | {} | {} | {} | {} | {} |",
        if with_footnotes { "[^1]" } else { "" },
        primary.num_regressions,
        secondary.num_regressions,
        primary.num_improvements,
        secondary.num_improvements,
        primary.num_regressions + primary.num_improvements
    )
    .unwrap();

    writeln!(
        result,
        "| mean{} | {} | {} | {} | {} | {} |",
        if with_footnotes { "[^2]" } else { "" },
        render_stat(primary.num_regressions, || Some(
            primary.arithmetic_mean_of_regressions()
        )),
        render_stat(secondary.num_regressions, || Some(
            secondary.arithmetic_mean_of_regressions()
        )),
        render_stat(primary.num_improvements, || Some(
            primary.arithmetic_mean_of_improvements()
        )),
        render_stat(secondary.num_improvements, || Some(
            secondary.arithmetic_mean_of_improvements()
        )),
        if primary.is_empty() {
            "N/A".to_string()
        } else {
            format!("{:.1}%", primary.arithmetic_mean_of_changes())
        }
    )
    .unwrap();

    let largest_change = if primary.is_empty() {
        "N/A".to_string()
    } else {
        let largest_improvement = primary
            .largest_improvement()
            .map(|c| c.relative_change())
            .unwrap_or(0.0);
        let largest_regression = primary
            .largest_regression()
            .map(|c| c.relative_change())
            .unwrap_or(0.0);
        let change = if largest_improvement
            .abs()
            .partial_cmp(&largest_regression.abs())
            .unwrap_or(Ordering::Equal)
            != Ordering::Less
        {
            largest_improvement
        } else {
            largest_regression
        };

        format!("{:.1}%", change * 100.0)
    };

    writeln!(
        result,
        "| max | {} | {} | {} | {} | {} |",
        render_stat(primary.num_regressions, || primary
            .largest_regression()
            .map(|r| r.relative_change() * 100.0)),
        render_stat(secondary.num_regressions, || secondary
            .largest_regression()
            .map(|r| r.relative_change() * 100.0)),
        render_stat(primary.num_improvements, || primary
            .largest_improvement()
            .map(|r| r.relative_change() * 100.0)),
        render_stat(secondary.num_improvements, || secondary
            .largest_improvement()
            .map(|r| r.relative_change() * 100.0)),
        largest_change
    )
    .unwrap();

    if with_footnotes {
        writeln!(
            result,
            r#"
[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*"#
        )
        .unwrap();
    }
}

/// The amount of confidence we have that a comparison actually represents a real
/// change in the performance characteristics.
#[derive(Clone, Copy, Debug)]
pub enum ComparisonConfidence {
    MaybeRelevant,
    ProbablyRelevant,
    DefinitelyRelevant,
}

impl ComparisonConfidence {
    pub fn is_definitely_relevant(self) -> bool {
        matches!(self, Self::DefinitelyRelevant)
    }

    pub fn is_atleast_probably_relevant(self) -> bool {
        matches!(self, Self::DefinitelyRelevant | Self::ProbablyRelevant)
    }
}

/// Compare two bounds on a given stat
///
/// Returns Ok(None) when no data for the end bound is present
pub async fn compare(
    start: Bound,
    end: Bound,
    stat: String,
    ctxt: &SiteCtxt,
) -> Result<Option<Comparison>, BoxedError> {
    let master_commits = &ctxt.get_master_commits().commits;

    compare_given_commits(start, end, stat, ctxt, master_commits).await
}

/// Compare two bounds on a given stat
async fn compare_given_commits(
    start: Bound,
    end: Bound,
    stat: String,
    ctxt: &SiteCtxt,
    master_commits: &[collector::MasterCommit],
) -> Result<Option<Comparison>, BoxedError> {
    let idx = ctxt.index.load();
    let a = ctxt
        .artifact_id_for_bound(start.clone(), true)
        .ok_or(format!("could not find start commit for bound {:?}", start))?;
    let b = match ctxt.artifact_id_for_bound(end.clone(), false) {
        Some(b) => b,
        None => return Ok(None),
    };
    let aids = Arc::new(vec![a.clone(), b.clone()]);

    // get all crates, cache, and profile combinations for the given stat
    let query = selector::Query::new()
        .set::<String>(Tag::Benchmark, selector::Selector::All)
        .set::<String>(Tag::Scenario, selector::Selector::All)
        .set::<String>(Tag::Profile, selector::Selector::All)
        .set(Tag::Metric, selector::Selector::One(stat.clone()));

    // `responses` contains series iterators. The first element in the iterator is the data
    // for `a` and the second is the data for `b`
    let mut responses = ctxt.statistic_series(query.clone(), aids).await?;

    let conn = ctxt.conn().await;
    let statistics_for_a = statistics_from_series(&mut responses);
    let statistics_for_b = statistics_from_series(&mut responses);

    let mut historical_data =
        HistoricalDataMap::calculate(ctxt, a.clone(), master_commits, stat).await?;
    let statistics = statistics_for_a
        .into_iter()
        .filter_map(|(test_case, a)| {
            statistics_for_b
                .get(&test_case)
                .map(|&b| TestResultComparison {
                    benchmark: test_case.0,
                    profile: test_case.1,
                    scenario: test_case.2,
                    historical_data: historical_data.data.remove(&test_case),
                    results: (a, b),
                })
        })
        .collect();

    let mut errors_in_b = conn.get_error(b.lookup(&idx).unwrap()).await;
    let errors_in_a = conn.get_error(a.lookup(&idx).unwrap()).await;
    for (name, _) in errors_in_a {
        errors_in_b.remove(&name);
    }

    Ok(Some(Comparison {
        a: ArtifactDescription::for_artifact(&*conn, a.clone(), master_commits).await,
        b: ArtifactDescription::for_artifact(&*conn, b.clone(), master_commits).await,
        statistics,
        new_errors: errors_in_b.into_iter().collect(),
    }))
}

fn previous_commits(
    mut from: ArtifactId,
    n: usize,
    master_commits: &[collector::MasterCommit],
) -> Vec<ArtifactId> {
    let mut prevs = Vec::with_capacity(n);
    while prevs.len() < n {
        match prev_commit(&from, master_commits) {
            Some(c) => {
                let new = ArtifactId::Commit(database::Commit {
                    sha: c.sha.clone(),
                    date: database::Date(c.time),
                });
                from = new.clone();
                prevs.push(new);
            }
            None => break,
        }
    }
    prevs
}

/// Detailed description of a specific artifact
#[derive(Debug, Clone)]
pub struct ArtifactDescription {
    /// The artifact in question
    pub artifact: ArtifactId,
    /// The pr of the artifact if known
    pub pr: Option<u32>,
    /// Bootstrap data in the form "$crate" -> nanoseconds
    pub bootstrap: HashMap<String, u64>,
    pub bootstrap_total: u64,
}

type StatisticsMap = HashMap<TestCase, f64>;
type TestCase = (Benchmark, Profile, Scenario);

impl ArtifactDescription {
    /// For the given `ArtifactId`, consume the first datapoint in each of the given `SeriesResponse`
    ///
    /// It is assumed that the provided `ArtifactId` matches the artifact id of the next data
    /// point for all of `SeriesResponse<T>`. If this is not true, this function will panic.
    async fn for_artifact(
        conn: &dyn database::Connection,
        artifact: ArtifactId,
        master_commits: &[collector::MasterCommit],
    ) -> Self {
        let bootstrap = conn
            .get_bootstrap_by_crate(&[conn.artifact_id(&artifact).await])
            .await;
        let bootstrap_total = bootstrap
            .values()
            .filter_map(|v| {
                v.get(0)
                    .copied()
                    .unwrap_or_default()
                    .map(|v| v.as_nanos() as u64)
            })
            .sum::<u64>();
        let bootstrap = bootstrap
            .into_iter()
            .filter_map(|(k, mut v)| {
                v.pop()
                    .unwrap_or_default()
                    // FIXME: if we're hovering right at the 1 second mark,
                    // this might mean we end up with a Some for one commit and
                    // a None for the other commit. Ultimately it doesn't matter
                    // that much -- we'll mostly just ignore such results.
                    // Anything less than a second in wall-time measurements is
                    // always going to be pretty high variance just from process
                    // startup overheads and such, though, so we definitely
                    // don't want to compare those values.
                    .filter(|v| v.as_secs() >= 1)
                    .map(|v| (k, v.as_nanos() as u64))
            })
            .collect::<HashMap<_, _>>();

        let pr = if let ArtifactId::Commit(c) = &artifact {
            if let Some(m) = master_commits.iter().find(|m| m.sha == c.sha) {
                m.pr
            } else {
                conn.pr_of(&c.sha).await
            }
        } else {
            None
        };

        Self {
            pr,
            artifact,
            bootstrap,
            bootstrap_total,
        }
    }
}

fn statistics_from_series<T>(series: &mut [selector::SeriesResponse<T>]) -> StatisticsMap
where
    T: Iterator<Item = (ArtifactId, Option<f64>)>,
{
    let mut stats: StatisticsMap = HashMap::new();
    for response in series {
        let (_, point) = response.series.next().expect("must have element");

        let value = if let Some(v) = point {
            v
        } else {
            continue;
        };
        let benchmark = *response.path.get::<Benchmark>().unwrap();
        let profile = *response.path.get::<Profile>().unwrap();
        let scenario = *response.path.get::<Scenario>().unwrap();
        stats.insert((benchmark, profile, scenario), value);
    }
    stats
}

impl From<ArtifactDescription> for api::comparison::ArtifactDescription {
    fn from(data: ArtifactDescription) -> Self {
        api::comparison::ArtifactDescription {
            commit: match data.artifact.clone() {
                ArtifactId::Commit(c) => c.sha,
                ArtifactId::Tag(t) => t,
            },
            date: if let ArtifactId::Commit(c) = &data.artifact {
                Some(c.date)
            } else {
                None
            },
            pr: data.pr,
            bootstrap: data.bootstrap,
            bootstrap_total: data.bootstrap_total,
        }
    }
}

// A comparison of two artifacts
#[derive(Clone)]
pub struct Comparison {
    pub a: ArtifactDescription,
    pub b: ArtifactDescription,
    /// Statistics based on test case
    pub statistics: HashSet<TestResultComparison>,
    pub new_errors: HashMap<String, String>,
}

impl Comparison {
    /// Gets the previous commit before `a`
    pub fn prev(&self, master_commits: &[collector::MasterCommit]) -> Option<String> {
        prev_commit(&self.a.artifact, master_commits).map(|c| c.sha.clone())
    }

    /// Determines if `a` and `b` are contiguous
    pub async fn is_contiguous(
        &self,
        conn: &dyn database::Connection,
        master_commits: &[collector::MasterCommit],
    ) -> bool {
        match (&self.a.artifact, &self.b.artifact) {
            (ArtifactId::Commit(a), ArtifactId::Commit(b)) => {
                if let Some(b) = master_commits.iter().find(|c| c.sha == b.sha) {
                    b.parent_sha == a.sha
                } else {
                    conn.parent_of(&b.sha).await.map_or(false, |p| p == a.sha)
                }
            }
            _ => false,
        }
    }

    /// Gets the sha of the next commit after `b`
    pub fn next(&self, master_commits: &[collector::MasterCommit]) -> Option<String> {
        next_commit(&self.b.artifact, master_commits).map(|c| c.sha.clone())
    }

    /// Splits comparison into primary and secondary summaries based on benchmark category
    pub fn summarize_by_category(
        self,
        map: HashMap<Benchmark, Category>,
    ) -> (Option<ComparisonSummary>, Option<ComparisonSummary>) {
        let (primary, secondary) = self
            .statistics
            .into_iter()
            .partition(|s| map.get(&s.benchmark()) == Some(&Category::Primary));

        let primary = Comparison {
            a: self.a.clone(),
            b: self.b.clone(),
            statistics: primary,
            new_errors: self.new_errors.clone(),
        };
        let secondary = Comparison {
            a: self.a,
            b: self.b,
            statistics: secondary,
            new_errors: self.new_errors,
        };
        (
            ComparisonSummary::summarize_comparison(&primary),
            ComparisonSummary::summarize_comparison(&secondary),
        )
    }
}

/// The historical data for a certain benchmark
pub struct HistoricalDataMap {
    /// Historical data on a per test case basis
    pub data: HashMap<(Benchmark, Profile, Scenario), HistoricalData>,
}

impl HistoricalDataMap {
    const NUM_PREVIOUS_COMMITS: usize = 100;
    const MIN_PREVIOUS_COMMITS: usize = 50;

    async fn calculate(
        ctxt: &SiteCtxt,
        from: ArtifactId,
        master_commits: &[collector::MasterCommit],
        stat: String,
    ) -> Result<Self, BoxedError> {
        let mut historical_data = HashMap::new();

        let previous_commits = Arc::new(previous_commits(
            from,
            Self::NUM_PREVIOUS_COMMITS,
            master_commits,
        ));

        // Return early if we don't have enough data for historical analysis
        if previous_commits.len() < Self::MIN_PREVIOUS_COMMITS {
            return Ok(Self {
                data: historical_data,
            });
        }

        // get all crates, cache, and profile combinations for the given stat
        let query = selector::Query::new()
            .set::<String>(Tag::Benchmark, selector::Selector::All)
            .set::<String>(Tag::Scenario, selector::Selector::All)
            .set::<String>(Tag::Profile, selector::Selector::All)
            .set(Tag::Metric, selector::Selector::One(stat));

        let mut previous_commit_series = ctxt
            .statistic_series(query, previous_commits.clone())
            .await?;

        for _ in previous_commits.iter() {
            for (test_case, stat) in statistics_from_series(&mut previous_commit_series) {
                historical_data.entry(test_case).or_default().push(stat);
            }
        }

        // Only retain test cases for which we have enough data to calculate variance.
        historical_data.retain(|_, v| v.data.len() >= Self::MIN_PREVIOUS_COMMITS);

        Ok(Self {
            data: historical_data,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HistoricalData {
    data: Vec<f64>,
}

impl HistoricalData {
    /// The multiple of the IQR above Q3 that signifies significance
    const IQR_MULTIPLIER: f64 = 3.0;

    fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    /// The percent change of the deltas sorted from smallest delta to largest
    fn percent_changes(&self) -> Vec<f64> {
        let mut deltas = self
            .deltas()
            .zip(self.data.iter())
            .map(|(d, &r)| d / r)
            .collect::<Vec<_>>();
        deltas.sort_by(|d1, d2| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal));
        deltas
    }

    // This is an absolute value indicating the noise barrier for changes on
    // this benchmark.
    //
    // A number line could be divided like this:
    //
    // ------o-------o----------
    //       ^   ^   ^
    //       |   |   |
    //       |   |   |
    //       |   |   ---- +significance_threshold
    //       |   |
    //       |   - not significant, includes zero
    //       |
    //       ---- -significance_threshold()
    fn significance_threshold(&self) -> f64 {
        let (q1, q3) = self.quartiles();

        // Changes that are IQR_MULTIPLIER away from the Q3 are considered
        // outliers, and we judge those as significant.
        q3 + (q3 - q1) * Self::IQR_MULTIPLIER
    }

    // (q1, q3)
    fn quartiles(&self) -> (f64, f64) {
        let pcs = self.percent_changes();
        fn median(data: &[f64]) -> f64 {
            if data.len() % 2 == 0 {
                (data[(data.len() - 1) / 2] + data[data.len() / 2]) / 2.0
            } else {
                data[data.len() / 2]
            }
        }

        let len = pcs.len();
        let (h1_end, h2_begin) = if len % 2 == 0 {
            (len / 2 - 2, len / 2 + 1)
        } else {
            (len / 2 - 1, len / 2 + 1)
        };
        let q1 = median(&pcs[..=h1_end]);
        let q3 = median(&pcs[h2_begin..]);

        (q1, q3)
    }

    // Absolute deltas between adjacent results
    fn deltas(&self) -> impl Iterator<Item = f64> + '_ {
        self.data
            .windows(2)
            .map(|window| (window[0] - window[1]).abs())
    }

    /// Whether we can trust this benchmark or not
    fn is_dodgy(&self) -> bool {
        // If changes are judged significant only exceeding 0.2%, then the
        // benchmark as a whole is dodgy.
        self.significance_threshold() * 100.0 > 0.2
    }
}

/// Gets the previous commit
pub fn prev_commit<'a>(
    artifact: &ArtifactId,
    master_commits: &'a [collector::MasterCommit],
) -> Option<&'a collector::MasterCommit> {
    match &artifact {
        ArtifactId::Commit(a) => {
            let current = master_commits.iter().find(|c| c.sha == a.sha)?;
            master_commits.iter().find(|c| c.sha == current.parent_sha)
        }
        ArtifactId::Tag(_) => None,
    }
}

/// Gets the next commit
pub fn next_commit<'a>(
    artifact: &ArtifactId,
    master_commits: &'a [collector::MasterCommit],
) -> Option<&'a collector::MasterCommit> {
    match artifact {
        ArtifactId::Commit(b) => master_commits.iter().find(|c| c.parent_sha == b.sha),
        ArtifactId::Tag(_) => None,
    }
}

// A single comparison between two test results
#[derive(Debug, Clone)]
pub struct TestResultComparison {
    benchmark: Benchmark,
    profile: Profile,
    scenario: Scenario,
    historical_data: Option<HistoricalData>,
    results: (f64, f64),
}

impl TestResultComparison {
    /// The amount of relative change considered significant when
    /// we cannot determine from historical data
    const DEFAULT_SIGNIFICANCE_THRESHOLD: f64 = 0.002;

    pub fn benchmark(&self) -> Benchmark {
        self.benchmark
    }

    fn is_regression(&self) -> bool {
        let (a, b) = self.results;
        b > a
    }

    fn is_improvement(&self) -> bool {
        !self.is_regression()
    }

    /// Whther the comparison yielded a statistically significant result
    pub fn is_significant(&self) -> bool {
        self.relative_change().abs() >= self.significance_threshold()
    }

    /// Magnitude of change considered significant
    fn significance_threshold(&self) -> f64 {
        self.historical_data
            .as_ref()
            .map(|d| d.significance_threshold())
            .unwrap_or(Self::DEFAULT_SIGNIFICANCE_THRESHOLD)
    }

    /// This is a numeric magnitude of a particular change.
    fn significance_factor(&self) -> Option<f64> {
        let change = self.relative_change();
        let threshold = self.significance_threshold();
        // How many times the treshold this change is.
        Some(change.abs() / threshold)
    }

    fn magnitude(&self) -> Magnitude {
        let change = self.relative_change().abs();
        let threshold = self.significance_threshold();
        let over_threshold = if change < threshold * 1.5 {
            Magnitude::VerySmall
        } else if change < threshold * 3.0 {
            Magnitude::Small
        } else if change < threshold * 6.0 {
            Magnitude::Medium
        } else if change < threshold * 12.0 {
            Magnitude::Large
        } else {
            Magnitude::VeryLarge
        };
        let change_magnitude = if change < 0.002 {
            Magnitude::VerySmall
        } else if change < 0.01 {
            Magnitude::Small
        } else if change < 0.02 {
            Magnitude::Medium
        } else if change < 0.05 {
            Magnitude::Large
        } else {
            Magnitude::VeryLarge
        };
        fn as_u8(m: Magnitude) -> u8 {
            match m {
                Magnitude::VerySmall => 1,
                Magnitude::Small => 2,
                Magnitude::Medium => 3,
                Magnitude::Large => 4,
                Magnitude::VeryLarge => 5,
            }
        }
        fn from_u8(m: u8) -> Magnitude {
            match m {
                1 => Magnitude::VerySmall,
                2 => Magnitude::Small,
                3 => Magnitude::Medium,
                4 => Magnitude::Large,
                _ => Magnitude::VeryLarge,
            }
        }

        from_u8((as_u8(over_threshold) + as_u8(change_magnitude)) / 2)
    }

    fn is_dodgy(&self) -> bool {
        self.historical_data
            .as_ref()
            .map(|v| v.is_dodgy())
            .unwrap_or(false)
    }

    fn relative_change(&self) -> f64 {
        let (a, b) = self.results;
        (b - a) / a
    }
}

impl std::cmp::PartialEq for TestResultComparison {
    fn eq(&self, other: &Self) -> bool {
        self.benchmark == other.benchmark
            && self.profile == other.profile
            && self.scenario == other.scenario
    }
}

impl std::cmp::Eq for TestResultComparison {}

impl std::hash::Hash for TestResultComparison {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.benchmark.hash(state);
        self.profile.hash(state);
        self.scenario.hash(state);
    }
}

// The direction of a performance change
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Improvement,
    Regression,
    Mixed,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Direction::Improvement => "improvement",
            Direction::Regression => "regression",
            Direction::Mixed => "mixed",
        };
        write!(f, "{}", description)
    }
}

/// The relative size of a performance change
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Magnitude {
    VerySmall,
    Small,
    Medium,
    Large,
    VeryLarge,
}

impl Magnitude {
    fn is_small_or_above(&self) -> bool {
        *self >= Self::Small
    }

    fn is_medium_or_above(&self) -> bool {
        *self >= Self::Medium
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::VerySmall => "very small",
            Self::Small => "small",
            Self::Medium => "moderate",
            Self::Large => "large",
            Self::VeryLarge => "very large",
        }
    }
}

async fn generate_report(
    start: &Bound,
    end: &Bound,
    mut report: HashMap<Option<Direction>, Vec<String>>,
    num_comparisons: usize,
) -> String {
    fn fmt_bound(bound: &Bound) -> String {
        match bound {
            Bound::Commit(s) => s.to_owned(),
            Bound::Date(s) => s.format("%Y-%m-%d").to_string(),
            _ => "???".to_owned(),
        }
    }
    let start = fmt_bound(start);
    let end = fmt_bound(end);
    let regressions = report
        .remove(&Some(Direction::Regression))
        .unwrap_or_default();
    let improvements = report
        .remove(&Some(Direction::Improvement))
        .unwrap_or_default();
    let mixed = report.remove(&Some(Direction::Mixed)).unwrap_or_default();
    let unlabeled = report.remove(&None).unwrap_or_default();
    let untriaged = match github::untriaged_perf_regressions().await {
        Ok(u) => u
            .iter()
            .map(|github::PullRequest { title, number }| {
                format!(
                    "- [#{} {}](https://github.com/rust-lang/rust/pull/{})",
                    number, title, number
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Err(e) => format!(
            "An **error** occurred when finding the untriaged PRs: {}",
            e
        ),
    };
    format!(
        r#####"# {date} Triage Log

TODO: Summary

Triage done by **@???**.
Revision range: [{first_commit}..{last_commit}](https://perf.rust-lang.org/?start={first_commit}&end={last_commit}&absolute=false&stat=instructions%3Au)

{num_regressions} Regressions, {num_improvements} Improvements, {num_mixed} Mixed; ??? of them in rollups
{num_comparisons} comparisons made in total

#### Regressions

{regressions}

#### Improvements

{improvements}

#### Mixed

{mixed}

#### Probably changed

The following is a list of comparisons which *probably* represent real performance changes,
but we're not 100% sure. Please move things from this category into the categories
above for changes you think *are* definitely relevant and file an issue for each so that
we can consider how to change our heuristics.

{unlabeled}

#### Untriaged Pull Requests

{untriaged}

#### Nags requiring follow up

TODO: Nags

"#####,
        date = chrono::Utc::today().format("%Y-%m-%d"),
        first_commit = start,
        last_commit = end,
        num_comparisons = num_comparisons,
        num_regressions = regressions.len(),
        num_improvements = improvements.len(),
        num_mixed = mixed.len(),
        regressions = regressions.join("\n\n"),
        improvements = improvements.join("\n\n"),
        mixed = mixed.join("\n\n"),
        unlabeled = unlabeled.join("\n\n"),
        untriaged = untriaged
    )
}

fn compare_link(start: &ArtifactId, end: &ArtifactId) -> String {
    let start = match &start {
        ArtifactId::Tag(a) => a,
        ArtifactId::Commit(c) => &c.sha,
    };
    let end = match &end {
        ArtifactId::Tag(a) => a,
        ArtifactId::Commit(c) => &c.sha,
    };
    format!(
        "https://perf.rust-lang.org/compare.html?start={}&end={}&stat=instructions:u",
        start, end
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use collector::category::Category;
    use std::collections::HashSet;

    use database::{ArtifactId, Profile, Scenario};

    #[test]
    fn summary_table_only_regressions_primary() {
        check_table(
            vec![
                (Category::Primary, 5.0, 10.0),
                (Category::Primary, 5.0, 12.0),
                (Category::Primary, 1.0, 3.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 3 | 0 | 0 | 0 | 3 |
| mean[^2] | 146.7% | N/A | N/A | N/A | 146.7% |
| max | 200.0% | N/A | N/A | N/A | 200.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
            .trim_start(),
        );
    }

    #[test]
    fn summary_table_only_improvements_primary() {
        check_table(
            vec![
                (Category::Primary, 5.0, 2.0),
                (Category::Primary, 5.0, 1.0),
                (Category::Primary, 4.0, 1.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 0 | 0 | 3 | 0 | 3 |
| mean[^2] | N/A | N/A | -71.7% | N/A | -71.7% |
| max | N/A | N/A | -80.0% | N/A | -80.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
            .trim_start(),
        );
    }

    #[test]
    fn summary_table_only_improvements_secondary() {
        check_table(
            vec![
                (Category::Secondary, 5.0, 2.0),
                (Category::Secondary, 5.0, 1.0),
                (Category::Secondary, 4.0, 1.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 0 | 0 | 0 | 3 | 0 |
| mean[^2] | N/A | N/A | N/A | -71.7% | N/A |
| max | N/A | N/A | N/A | -80.0% | N/A |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
                .trim_start(),
        );
    }

    #[test]
    fn summary_table_only_regressions_secondary() {
        check_table(
            vec![
                (Category::Secondary, 5.0, 10.0),
                (Category::Secondary, 5.0, 12.0),
                (Category::Secondary, 1.0, 3.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 0 | 3 | 0 | 0 | 0 |
| mean[^2] | N/A | 146.7% | N/A | N/A | N/A |
| max | N/A | 200.0% | N/A | N/A | N/A |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
                .trim_start(),
        );
    }

    #[test]
    fn summary_table_mixed_primary() {
        check_table(
            vec![
                (Category::Primary, 10.0, 5.0),
                (Category::Primary, 5.0, 10.0),
                (Category::Primary, 1.0, 3.0),
                (Category::Primary, 4.0, 1.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 2 | 0 | 2 | 0 | 4 |
| mean[^2] | 150.0% | N/A | -62.5% | N/A | 43.8% |
| max | 200.0% | N/A | -75.0% | N/A | 200.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
            .trim_start(),
        );
    }

    #[test]
    fn summary_table_mixed_primary_secondary() {
        check_table(
            vec![
                (Category::Primary, 10.0, 5.0),
                (Category::Primary, 5.0, 10.0),
                (Category::Secondary, 5.0, 10.0),
                (Category::Primary, 1.0, 3.0),
                (Category::Secondary, 3.0, 1.0),
                (Category::Primary, 4.0, 1.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 2 | 1 | 2 | 1 | 4 |
| mean[^2] | 150.0% | 100.0% | -62.5% | -66.7% | 43.8% |
| max | 200.0% | 100.0% | -75.0% | -66.7% | 200.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
                .trim_start(),
        );
    }

    #[test]
    fn summary_table_mixed_largest_change_improvement() {
        check_table(
            vec![
                (Category::Primary, 10.0, 5.0),
                (Category::Primary, 5.0, 6.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 1 | 0 | 1 | 0 | 2 |
| mean[^2] | 20.0% | N/A | -50.0% | N/A | -15.0% |
| max | 20.0% | N/A | -50.0% | N/A | -50.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
                .trim_start(),
        );
    }

    #[test]
    fn summary_table_mixed_largest_change_regression() {
        check_table(
            vec![
                (Category::Primary, 5.0, 10.0),
                (Category::Primary, 6.0, 5.0),
            ],
            r#"
| | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:---:|:---:|:---:|:---:|:---:|:---:|
| count[^1] | 1 | 0 | 1 | 0 | 2 |
| mean[^2] | 100.0% | N/A | -16.7% | N/A | 41.7% |
| max | 100.0% | N/A | -16.7% | N/A | 100.0% |

[^1]: *number of relevant changes*
[^2]: *the arithmetic mean of the percent change*
"#
                .trim_start(),
        );
    }

    // (category, before, after)
    fn check_table(values: Vec<(Category, f64, f64)>, expected: &str) {
        let mut primary_statistics = HashSet::new();
        let mut secondary_statistics = HashSet::new();
        for (index, (category, before, after)) in values.into_iter().enumerate() {
            let target = if category == Category::Primary {
                &mut primary_statistics
            } else {
                &mut secondary_statistics
            };

            target.insert(TestResultComparison {
                benchmark: index.to_string().as_str().into(),
                profile: Profile::Check,
                scenario: Scenario::Empty,
                historical_data: None,
                results: (before, after),
            });
        }

        let primary = create_summary(primary_statistics);
        let secondary = create_summary(secondary_statistics);

        let mut result = String::new();
        write_summary_table(&primary, &secondary, true, &mut result);
        assert_eq!(result, expected);
    }

    fn create_summary(statistics: HashSet<TestResultComparison>) -> ComparisonSummary {
        let comparison = Comparison {
            a: ArtifactDescription {
                artifact: ArtifactId::Tag("a".to_string()),
                pr: None,
                bootstrap: Default::default(),
                bootstrap_total: 0,
            },
            b: ArtifactDescription {
                artifact: ArtifactId::Tag("b".to_string()),
                pr: None,
                bootstrap: Default::default(),
                bootstrap_total: 0,
            },
            statistics,
            new_errors: Default::default(),
        };
        ComparisonSummary::summarize_comparison(&comparison)
            .unwrap_or_else(|| ComparisonSummary::empty())
    }
}
