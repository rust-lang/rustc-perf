//! Functionality for comparing
//! This is mainly used to build the triage report and the perf
//! comparison endpoints

use crate::api;
use crate::github;
use crate::load::SiteCtxt;

use collector::compile::benchmark::category::Category;
use collector::Bound;
use database::{
    metric::Metric,
    selector::{self, BenchmarkQuery, CompileBenchmarkQuery, RuntimeBenchmarkQuery, TestCase},
};
use database::{ArtifactId, Benchmark, Lookup, Profile, Scenario};
use serde::Serialize;

use crate::api::comparison::CompileBenchmarkMetadata;
use crate::benchmark_metadata::get_compile_benchmarks_metadata;
use crate::server::comparison::StatComparison;
use collector::compile::benchmark::ArtifactType;
use database::{CodegenBackend, CommitType, CompileBenchmark};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write;
use std::hash::Hash;
use std::iter;
use std::ops::Deref;
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
    let end_artifact = ctxt
        .artifact_id_for_bound(end.clone(), false)
        .ok_or(format!("could not find end commit for bound {:?}", end))?;
    // This gives a better error, but is still not great -- the common case here
    // is that we've had a 422 error and as such had a fork. It's possible we
    // could diagnose that and give a nicer error here telling the user which
    // commit to use.
    let mut next = next_commit(&start_artifact, master_commits)
        .map(|c| Bound::Commit(c.sha.clone()))
        .ok_or(format!("no next commit for {:?}", start_artifact))?;

    let mut report = HashMap::new();
    let mut before = start.clone();

    let mut num_comparisons = 0;
    let metric = body.metric.unwrap_or(Metric::InstructionsUser);
    let benchmark_map = ctxt.get_benchmark_category_map().await;

    let end = loop {
        let comparison =
            match compare_given_commits(before.clone(), next.clone(), metric, ctxt, master_commits)
                .await
                .map_err(|e| format!("error comparing commits: {}", e))?
            {
                Some(c) => c,
                None => {
                    log::info!(
                        "No data found for end bound {:?}. Ending comparison...",
                        next
                    );
                    break before;
                }
            };
        num_comparisons += 1;
        log::info!(
            "Comparing {} to {}",
            comparison.b.artifact,
            comparison.a.artifact
        );

        // handle results of comparison
        populate_report(&comparison, &benchmark_map, metric, &mut report).await;

        // If we already know this is the last iteration, we can stop
        if comparison.b.artifact == end_artifact {
            break before;
        }

        // Decide whether to keep doing comparisons or not
        match comparison.next(master_commits).map(Bound::Commit) {
            // There is a next commit, and it is not the end bound.
            // We keep doing comparisons...
            Some(n) => {
                before = next;
                next = n;
            }
            // There is no next commit so we stop.
            None => break before,
        }
    };

    // Summarize the entire triage from start commit to end commit
    let summary =
        match compare_given_commits(start.clone(), end.clone(), metric, ctxt, master_commits)
            .await
            .map_err(|e| format!("error comparing beginning and ending commits: {}", e))?
        {
            Some(summary_comparison) => {
                let (primary, secondary) =
                    summary_comparison.summarize_compile_by_category(&benchmark_map);
                let mut result = String::from("**Summary**:\n\n");
                write_summary_table(&primary, &secondary, true, &mut result);
                result
            }
            None => String::from("**ERROR**: no data found for end bound"),
        };

    let report = generate_report(&start, &end, summary, report, num_comparisons).await;
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
    let compile_benchmark_map = conn.get_compile_benchmarks().await;

    let compile_comparisons = comparison
        .compile_comparisons
        .into_iter()
        .map(|comparison| api::comparison::CompileBenchmarkComparison {
            benchmark: comparison.benchmark.to_string(),
            profile: comparison.profile.to_string(),
            scenario: comparison.scenario.to_string(),
            backend: comparison.backend.to_string(),
            comparison: comparison.comparison.into(),
        })
        .collect();

    let runtime_comparisons = comparison
        .runtime_comparisons
        .into_iter()
        .map(|comparison| api::comparison::RuntimeBenchmarkComparison {
            benchmark: comparison.benchmark.to_string(),
            comparison: comparison.comparison.into(),
        })
        .collect();

    let mut new_errors = comparison
        .newly_failed_benchmarks
        .into_iter()
        .collect::<Vec<_>>();
    new_errors.sort();

    let compile_metadata = get_compile_benchmarks_metadata();
    // Enrich data from the DB with metadata generated by the build script
    let compile_benchmark_metadata = compile_benchmark_map
        .into_iter()
        .map(|benchmark| {
            let CompileBenchmark { name, category } = benchmark;
            let metadata = compile_metadata.get(&name);

            CompileBenchmarkMetadata {
                name,
                category,
                binary: metadata.map(|m| m.perf_config.artifact() == ArtifactType::Binary),
                iterations: metadata.map(|m| m.perf_config.iterations() as u32),
                release_profile: metadata.map(|m| m.release_metadata.clone()),
                dev_profile: metadata.map(|m| m.dev_metadata.clone()),
            }
        })
        .collect();

    Ok(api::comparison::Response {
        prev,
        a: comparison.a.into(),
        b: comparison.b.into(),
        compile_comparisons,
        runtime_comparisons,
        new_errors,
        next,
        is_contiguous,
        compile_benchmark_metadata,
    })
}

async fn populate_report(
    comparison: &ArtifactComparison,
    benchmark_map: &HashMap<Benchmark, Category>,
    metric: Metric,
    report: &mut HashMap<Direction, Vec<String>>,
) {
    let (primary, secondary) = comparison
        .clone()
        .summarize_compile_by_category(benchmark_map);
    // Get the combined direction of the primary and secondary summaries
    let direction = Direction::join(primary.direction(), secondary.direction());
    if direction == Direction::None {
        return;
    }

    let include_in_triage = match metric {
        Metric::InstructionsUser => deserves_attention_icount(&primary, &secondary),
        _ => primary
            .largest_change()
            .or_else(|| secondary.largest_change())
            .map(|c| c.magnitude() >= Magnitude::Small)
            .unwrap_or(false),
    };

    if include_in_triage {
        let entry = report.entry(direction).or_default();
        entry.push(write_triage_summary(comparison, &primary, &secondary).await);
    }
}

trait MetricExt {
    fn relative_change_magnitude(&self, change: f64) -> Magnitude;
    fn is_typically_noisy(&self) -> bool;
}

impl MetricExt for Metric {
    /// Determines the magnitude of a percent relative change for a given metric.
    ///
    /// Takes into account how noisy the stat is. For example, instruction
    /// count which is normally not very noisy has smaller thresholds than
    /// max-rss which can be noisy.
    fn relative_change_magnitude(&self, change: f64) -> Magnitude {
        let noise_factor = if self.is_typically_noisy() { 2.0 } else { 1.0 };
        let change = change / noise_factor;
        if change < 0.2 {
            Magnitude::VerySmall
        } else if change < 1.0 {
            Magnitude::Small
        } else if change < 2.0 {
            Magnitude::Medium
        } else if change < 5.0 {
            Magnitude::Large
        } else {
            Magnitude::VeryLarge
        }
    }

    /// Currently, we conservatively consider everything except instructions to be noisy.
    fn is_typically_noisy(&self) -> bool {
        !matches!(self, Self::InstructionsUser)
    }
}

/// A summary of a given comparison
///
/// This summary only includes changes that are significant and relevant (as determined by a change's magnitude).
pub struct ArtifactComparisonSummary {
    /// Relevant comparisons ordered from most negative to most positive
    relevant_comparisons: Vec<TestResultComparison>,
    /// The cached number of comparisons that are improvements
    num_improvements: usize,
    /// The cached number of comparisons that are regressions
    num_regressions: usize,
}

impl ArtifactComparisonSummary {
    /// Summarize a collection of `TestResultComparison`
    pub fn summarize(comparisons: Vec<TestResultComparison>) -> Self {
        let mut num_improvements = 0;
        let mut num_regressions = 0;

        let mut relevant_comparisons = comparisons
            .into_iter()
            .filter(|c| c.is_relevant())
            .inspect(|c| {
                if c.is_improvement() {
                    num_improvements += 1;
                } else {
                    num_regressions += 1
                }
            })
            .collect::<Vec<_>>();

        let cmp = |b1: &TestResultComparison, b2: &TestResultComparison| {
            b1.relative_change()
                .partial_cmp(&b2.relative_change())
                .unwrap_or(cmp::Ordering::Equal)
        };
        relevant_comparisons.sort_by(cmp);

        ArtifactComparisonSummary {
            relevant_comparisons,
            num_improvements,
            num_regressions,
        }
    }

    /// The direction of the changes
    pub fn direction(&self) -> Direction {
        if self.relevant_comparisons.is_empty() {
            return Direction::None;
        }

        let (regressions, improvements): (Vec<&TestResultComparison>, _) = self
            .relevant_comparisons
            .iter()
            .partition(|c| c.is_regression());

        if regressions.is_empty() {
            return Direction::Improvement;
        }

        if improvements.is_empty() {
            return Direction::Regression;
        }

        let total_num = self.relevant_comparisons.len();
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
            (true, true) => Direction::Mixed,
            (true, false) => {
                if regressions_ratio >= 0.15 {
                    Direction::Mixed
                } else {
                    Direction::Improvement
                }
            }
            (false, true) => {
                if regressions_ratio < 0.85 {
                    Direction::Mixed
                } else {
                    Direction::Regression
                }
            }
            (false, false) => {
                if (0.1..=0.9).contains(&regressions_ratio) {
                    Direction::Mixed
                } else if regressions_ratio <= 0.1 {
                    Direction::Improvement
                } else {
                    Direction::Regression
                }
            }
        }
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
        self.arithmetic_mean(self.relevant_comparisons.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.relevant_comparisons.is_empty()
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
        self.relevant_comparisons
            .iter()
            .filter(|c| c.is_improvement())
    }

    fn regressions(&self) -> impl Iterator<Item = &TestResultComparison> {
        self.relevant_comparisons
            .iter()
            .filter(|c| c.is_regression())
    }

    // This is the most negative result.
    fn largest_improvement(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons
            .iter()
            .find(|s| s.is_improvement())
    }

    // This is the least negative result.
    fn smallest_improvement(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons
            .iter()
            .rfind(|s| s.is_improvement())
    }

    // This is the most positive result.
    fn largest_regression(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons
            .iter()
            .rfind(|s| s.is_regression())
    }

    // This is the least positive result.
    fn smallest_regression(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons.iter().find(|s| s.is_regression())
    }

    // This may be an improvement or a regression.
    fn most_positive_change(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons.last()
    }

    // This may be an improvement or a regression.
    fn most_negative_change(&self) -> Option<&TestResultComparison> {
        self.relevant_comparisons.first()
    }

    /// The relevance level of the entire comparison
    pub fn is_relevant(&self) -> bool {
        !self.is_empty()
    }

    pub fn num_changes(&self) -> usize {
        self.relevant_comparisons.len()
    }

    pub fn largest_change(&self) -> Option<&TestResultComparison> {
        if self.num_changes() == 0 {
            None
        } else {
            let most_pos = self.most_positive_change().unwrap();
            let most_neg = self.most_negative_change().unwrap();
            let most_pos_abs = most_pos.relative_change().abs();
            let most_neg_abs = most_neg.relative_change().abs();
            if most_neg_abs.partial_cmp(&most_pos_abs) == Some(cmp::Ordering::Greater) {
                Some(most_neg)
            } else {
                Some(most_pos)
            }
        }
    }
}

/// Whether we are confident enough that an artifact comparison represents a real change and thus deserves to be looked at.
///
/// For example, this can be used to determine if artifact comparisons with regressions should be labeled with the
/// `perf-regression` GitHub label or should be shown in the perf triage report.
pub(crate) fn deserves_attention_icount(
    primary: &ArtifactComparisonSummary,
    secondary: &ArtifactComparisonSummary,
) -> bool {
    match (primary.largest_change(), secondary.largest_change()) {
        (Some(c), _) if c.magnitude() >= Magnitude::Medium => true,
        (_, Some(c)) if c.magnitude() >= Magnitude::Large => true,
        _ => {
            // How we determine whether a group of small changes deserves attention is and always will be arbitrary,
            // but this feels good enough for now. We may choose in the future to become more sophisticated about it.
            let primary_n = primary.num_changes();
            let secondary_n = secondary.num_changes();
            (primary_n * 3 + secondary_n) >= 9
        }
    }
}

async fn write_triage_summary(
    comparison: &ArtifactComparison,
    primary: &ArtifactComparisonSummary,
    secondary: &ArtifactComparisonSummary,
) -> String {
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
    write!(&mut result, " [(Comparison Link)]({})\n\n", link).unwrap();

    write_summary_table(primary, secondary, true, &mut result);

    result
}

/// Writes a Markdown table containing summary of relevant results.
pub fn write_summary_table(
    primary: &ArtifactComparisonSummary,
    secondary: &ArtifactComparisonSummary,
    include_metric: bool,
    result: &mut String,
) {
    let metric = include_metric
        .then(|| {
            primary
                .relevant_comparisons
                .first()
                .or(secondary.relevant_comparisons.first())
                .map(|m| format!("({})", m.metric.as_str()))
        })
        .flatten()
        .unwrap_or_else(|| "          ".to_string());

    fn render_stat<F: FnOnce() -> Option<f64>>(count: usize, calculate: F) -> String {
        let value = if count > 0 { calculate() } else { None };
        value
            .map(|value| format!("{value:.1}%"))
            .unwrap_or_else(|| "-".to_string())
    }

    fn render_range<F: FnOnce() -> (f64, f64)>(count: usize, calculate: F) -> String {
        if count > 0 {
            let (a, b) = calculate();
            format!("[{a:.1}%, {b:.1}%]")
        } else {
            "-".to_string()
        }
    }

    // (label, mean, max, count)
    let mut columns = vec![];

    // label
    columns.push(vec![
        "Regressions ❌ <br /> (primary)".to_string(),
        "Regressions ❌ <br /> (secondary)".to_string(),
        "Improvements ✅ <br /> (primary)".to_string(),
        "Improvements ✅ <br /> (secondary)".to_string(),
        "All ❌✅ (primary)".to_string(),
    ]);

    // mean
    columns.push(vec![
        render_stat(primary.num_regressions, || {
            Some(primary.arithmetic_mean_of_regressions())
        }),
        render_stat(secondary.num_regressions, || {
            Some(secondary.arithmetic_mean_of_regressions())
        }),
        render_stat(primary.num_improvements, || {
            Some(primary.arithmetic_mean_of_improvements())
        }),
        render_stat(secondary.num_improvements, || {
            Some(secondary.arithmetic_mean_of_improvements())
        }),
        if primary.is_empty() {
            "-".to_string()
        } else {
            format!("{:.1}%", primary.arithmetic_mean_of_changes())
        },
    ]);

    // range
    let rel_change = |r: Option<&TestResultComparison>| r.unwrap().relative_change() * 100.0;
    columns.push(vec![
        render_range(primary.num_regressions, || {
            (
                rel_change(primary.smallest_regression()),
                rel_change(primary.largest_regression()),
            )
        }),
        render_range(secondary.num_regressions, || {
            (
                rel_change(secondary.smallest_regression()),
                rel_change(secondary.largest_regression()),
            )
        }),
        render_range(primary.num_improvements, || {
            (
                rel_change(primary.largest_improvement()),
                rel_change(primary.smallest_improvement()),
            )
        }),
        render_range(secondary.num_improvements, || {
            (
                rel_change(secondary.largest_improvement()),
                rel_change(secondary.smallest_improvement()),
            )
        }),
        render_range(primary.num_regressions + primary.num_improvements, || {
            (
                rel_change(primary.most_negative_change()),
                rel_change(primary.most_positive_change()),
            )
        }),
    ]);

    // count
    columns.push(vec![
        primary.num_regressions.to_string(),
        secondary.num_regressions.to_string(),
        primary.num_improvements.to_string(),
        secondary.num_improvements.to_string(),
        (primary.num_regressions + primary.num_improvements).to_string(),
    ]);

    // This code attempts to space the table cells evenly so that the data is
    // easy to read for anyone who is viewing the Markdown source.
    let column_labels = [
        metric,
        "mean".to_string(),
        "range".to_string(),
        "count".to_string(),
    ];

    // Calculate the console width of a string, allowing for double-width
    // chars. The `unicode_width` crate does this properly, but with only two
    // emoji in use we can do it easily ourselves.
    let width = |s: &str| {
        s.chars()
            .map(|c| if c == '❌' || c == '✅' { 2 } else { 1 })
            .sum()
    };

    // Get the width of each column.
    let column_widths: Vec<usize> = column_labels
        .iter()
        .zip(columns.iter())
        .map(|(column_label, column)| {
            // Get the maximum width of the column data cells and the column label.
            column
                .iter()
                .chain(iter::once(column_label))
                .map(|cell| width(cell))
                .max()
                .unwrap()
        })
        .collect();

    // Write column labels.
    for (column_label, &column_width) in column_labels.iter().zip(column_widths.iter()) {
        write!(
            result,
            "| {}{} ",
            column_label,
            " ".repeat(column_width - width(column_label))
        )
        .unwrap();
    }
    result.push_str("|\n");

    // Write lines under the column labels.
    for &column_width in &column_widths {
        write!(result, "|:{}:", "-".repeat(column_width)).unwrap();
    }
    result.push_str("|\n");

    // Write the column data.
    for row_idx in 0..5 {
        let row = columns.iter().map(|rows| rows[row_idx].clone());
        assert_eq!(row.len(), column_labels.len());
        for (cell, &column_width) in row.zip(column_widths.iter()) {
            write!(
                result,
                "| {}{} ",
                cell,
                " ".repeat(column_width - width(&cell))
            )
            .unwrap();
        }
        result.push_str("|\n");
    }
}

/// Compare two bounds on a given stat
///
/// Returns Ok(None) when no data for the end bound is present
pub async fn compare(
    start: Bound,
    end: Bound,
    metric: Metric,
    ctxt: &SiteCtxt,
) -> Result<Option<ArtifactComparison>, BoxedError> {
    let master_commits = &ctxt.get_master_commits().commits;

    compare_given_commits(start, end, metric, ctxt, master_commits).await
}

/// Compare two bounds on a given metric
async fn compare_given_commits(
    start: Bound,
    end: Bound,
    metric: Metric,
    ctxt: &SiteCtxt,
    master_commits: &[collector::MasterCommit],
) -> Result<Option<ArtifactComparison>, BoxedError> {
    let idx = ctxt.index.load();
    let a = ctxt
        .artifact_id_for_bound(start.clone(), true)
        .ok_or(format!("could not find start commit for bound {:?}", start))?;
    let b = match ctxt.artifact_id_for_bound(end.clone(), false) {
        Some(b) => b,
        None => return Ok(None),
    };
    let aids = Arc::new(vec![a.clone(), b.clone()]);

    // get all crates, cache, and profile combinations for the given metric
    let compile_comparisons = get_comparison::<CompileTestResultComparison, _, _>(
        ctxt,
        CompileBenchmarkQuery::all_for_metric(metric),
        a.clone(),
        aids.clone(),
        metric,
        master_commits,
        |test_case, comparison| CompileTestResultComparison {
            profile: test_case.profile,
            scenario: test_case.scenario,
            benchmark: test_case.benchmark,
            backend: test_case.backend,
            comparison,
        },
    )
    .await?;

    // get all crates, cache, and profile combinations for the given metric
    let runtime_comparisons = get_comparison::<RuntimeTestResultComparison, _, _>(
        ctxt,
        RuntimeBenchmarkQuery::all_for_metric(metric),
        a.clone(),
        aids,
        metric,
        master_commits,
        |test_case, comparison| RuntimeTestResultComparison {
            benchmark: test_case.benchmark,
            comparison,
        },
    )
    .await?;

    let conn = ctxt.conn().await;
    let mut errors_in_b = conn.get_error(b.lookup(&idx).unwrap()).await;
    let errors_in_a = conn.get_error(a.lookup(&idx).unwrap()).await;
    for (name, _) in errors_in_a {
        errors_in_b.remove(&name);
    }

    Ok(Some(ArtifactComparison {
        a: ArtifactDescription::for_artifact(&*conn, a.clone(), master_commits).await,
        b: ArtifactDescription::for_artifact(&*conn, b.clone(), master_commits).await,
        compile_comparisons,
        runtime_comparisons,
        newly_failed_benchmarks: errors_in_b.into_iter().collect(),
    }))
}

async fn get_comparison<
    Comparison: Eq + Hash,
    Query: BenchmarkQuery,
    F: Fn(Query::TestCase, TestResultComparison) -> Comparison,
>(
    ctxt: &SiteCtxt,
    query: Query,
    start_artifact: ArtifactId,
    aids: Arc<Vec<ArtifactId>>,
    metric: Metric,
    master_commits: &[collector::MasterCommit],
    func: F,
) -> Result<HashSet<Comparison>, BoxedError> {
    // `responses` contains series iterators. The first element in the iterator is the data
    // for `a` and the second is the data for `b`
    let mut responses = ctxt.statistic_series(query.clone(), aids).await?;

    let statistics_for_a = statistics_from_series(&mut responses);
    let statistics_for_b = statistics_from_series(&mut responses);

    let mut historical_data =
        HistoricalDataMap::<Query>::calculate(ctxt, start_artifact, master_commits, query).await?;
    Ok(statistics_for_a
        .into_iter()
        .filter_map(|(test_case, a)| {
            statistics_for_b.get(&test_case).map(|&b| {
                let comparison = TestResultComparison {
                    metric,
                    historical_data: historical_data.data.remove(&test_case),
                    results: (a, b),
                };
                func(test_case, comparison)
            })
        })
        .collect())
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
                    r#type: CommitType::Master,
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
    pub component_sizes: HashMap<String, u64>,
}

type StatisticsMap<TestCase> = HashMap<TestCase, f64>;

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
        let aid = conn.artifact_id(&artifact).await;
        let bootstrap = conn.get_bootstrap_by_crate(&[aid]).await;
        let bootstrap_total = bootstrap
            .values()
            .filter_map(|v| {
                v.first()
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

        let component_sizes = conn.get_artifact_size(aid).await.into_iter().collect();

        Self {
            pr,
            artifact,
            bootstrap,
            bootstrap_total,
            component_sizes,
        }
    }
}

fn statistics_from_series<Case: TestCase, T>(
    series: &mut [selector::SeriesResponse<Case, T>],
) -> StatisticsMap<Case>
where
    T: Iterator<Item = (ArtifactId, Option<f64>)>,
{
    let mut stats: StatisticsMap<Case> = HashMap::new();
    for response in series {
        let (_, point) = response.series.next().expect("must have element");

        let value = if let Some(v) = point {
            v
        } else {
            continue;
        };
        stats.insert(response.test_case.clone(), value);
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
            r#type: match &data.artifact {
                ArtifactId::Commit(c) => match c.r#type {
                    CommitType::Master => api::comparison::CommitType::Master,
                    CommitType::Try => api::comparison::CommitType::Try,
                },
                ArtifactId::Tag(_) => api::comparison::CommitType::Master,
            },
            date: if let ArtifactId::Commit(c) = &data.artifact {
                Some(c.date)
            } else {
                None
            },
            pr: data.pr,
            bootstrap: data.bootstrap,
            bootstrap_total: data.bootstrap_total,
            component_sizes: data.component_sizes,
        }
    }
}

// A comparison of two artifacts
#[derive(Clone)]
pub struct ArtifactComparison {
    pub a: ArtifactDescription,
    pub b: ArtifactDescription,
    /// Compile test result comparisons between the two artifacts
    pub compile_comparisons: HashSet<CompileTestResultComparison>,
    /// Runtime test result copmarisons between the two artifacts
    pub runtime_comparisons: HashSet<RuntimeTestResultComparison>,
    /// A map from benchmark name to an error which occured when building `b` but not `a`.
    pub newly_failed_benchmarks: HashMap<String, String>,
}

impl ArtifactComparison {
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

    /// Splits an artifact comparison into primary and secondary summaries based on benchmark category
    pub fn summarize_compile_by_category(
        self,
        category_map: &HashMap<Benchmark, Category>,
    ) -> (ArtifactComparisonSummary, ArtifactComparisonSummary) {
        let (primary, secondary): (
            Vec<CompileTestResultComparison>,
            Vec<CompileTestResultComparison>,
        ) = self
            .compile_comparisons
            .into_iter()
            .partition(|s| category_map.get(&s.benchmark()) == Some(&Category::Primary));
        (
            ArtifactComparisonSummary::summarize(
                primary
                    .into_iter()
                    .map(|comparison| comparison.comparison)
                    .collect(),
            ),
            ArtifactComparisonSummary::summarize(
                secondary
                    .into_iter()
                    .map(|comparison| comparison.comparison)
                    .collect(),
            ),
        )
    }
}

/// The historical data for a certain benchmark
pub struct HistoricalDataMap<Query: BenchmarkQuery> {
    /// Historical data on a per test case basis
    pub data: HashMap<Query::TestCase, HistoricalData>,
}

impl<Query: BenchmarkQuery> HistoricalDataMap<Query> {
    const NUM_PREVIOUS_COMMITS: usize = 30;

    async fn calculate(
        ctxt: &SiteCtxt,
        from: ArtifactId,
        master_commits: &[collector::MasterCommit],
        query: Query,
    ) -> Result<Self, BoxedError> {
        let mut historical_data = HashMap::new();

        let previous_commits = Arc::new(previous_commits(
            from,
            Self::NUM_PREVIOUS_COMMITS,
            master_commits,
        ));

        // Return early if we don't have enough data for historical analysis
        if previous_commits.len() < Self::NUM_PREVIOUS_COMMITS {
            return Ok(Self {
                data: historical_data,
            });
        }

        let mut previous_commit_series = ctxt
            .statistic_series(query, previous_commits.clone())
            .await?;

        for _ in previous_commits.iter() {
            for (key, stat) in statistics_from_series(&mut previous_commit_series) {
                historical_data.entry(key).or_default().push(stat);
            }
        }

        // Only retain test cases for which we have enough data to calculate variance.
        historical_data.retain(|_, v| v.data.len() >= Self::NUM_PREVIOUS_COMMITS);

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
        deltas.sort_by(|d1, d2| d1.partial_cmp(d2).unwrap_or(cmp::Ordering::Equal));
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
        let (h1_end, h2_begin) = if len <= 2 {
            (0, std::cmp::min(len, 1))
        } else if len % 2 == 0 {
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
    metric: Metric,
    historical_data: Option<HistoricalData>,
    results: (f64, f64),
}

impl TestResultComparison {
    /// The amount of relative change considered significant when
    /// we cannot determine from historical data
    const DEFAULT_SIGNIFICANCE_THRESHOLD: f64 = 0.002;

    fn is_regression(&self) -> bool {
        let (a, b) = self.results;
        b > a
    }

    fn is_improvement(&self) -> bool {
        !self.is_regression()
    }

    /// Whether the comparison yielded a statistically significant result
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
    fn significance_factor(&self) -> f64 {
        let change = self.relative_change();
        let threshold = self.significance_threshold();

        // How many times the threshold this change is.
        let factor = change.abs() / threshold;
        if factor.is_finite() {
            factor
        } else {
            0.0
        }
    }

    /// Whether the comparison is relevant or not.
    ///
    /// Relevance is a function of significance and magnitude.
    fn is_relevant(&self) -> bool {
        self.is_significant() && self.magnitude().is_small_or_above()
    }

    /// The magnitude of the change.
    ///
    /// This is the average of the absolute magnitude of the change
    /// and the amount above the significance threshold.
    pub fn magnitude(&self) -> Magnitude {
        let change = self.relative_change().abs();

        // When the significance threshold is very small, magnitude can become VeryLarge even though
        // the change itself if incredibly small. So we deliberately return a VerySmall magnitude
        // here to avoid marking such small result as being relevant.
        if change < 0.0001 {
            return Magnitude::VerySmall;
        }

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
        let absolute_magnitude = self.metric.relative_change_magnitude(change * 100.0);
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

        // Take the average of the absolute magnitude and the magnitude
        // above the significance threshold.
        from_u8((as_u8(over_threshold) + as_u8(absolute_magnitude)) / 2)
    }

    fn relative_change(&self) -> f64 {
        let (a, b) = self.results;
        (b - a) / a
    }
}

impl From<TestResultComparison> for StatComparison {
    fn from(comparison: TestResultComparison) -> Self {
        Self {
            is_relevant: comparison.is_relevant(),
            significance_threshold: comparison.significance_threshold(),
            significance_factor: comparison.significance_factor(),
            statistics: comparison.results,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompileTestResultComparison {
    benchmark: Benchmark,
    profile: Profile,
    scenario: Scenario,
    backend: CodegenBackend,
    comparison: TestResultComparison,
}

impl CompileTestResultComparison {
    pub fn benchmark(&self) -> Benchmark {
        self.benchmark
    }
}

impl cmp::PartialEq for CompileTestResultComparison {
    fn eq(&self, other: &Self) -> bool {
        self.benchmark == other.benchmark
            && self.profile == other.profile
            && self.scenario == other.scenario
            && self.backend == other.backend
    }
}

impl cmp::Eq for CompileTestResultComparison {}

impl std::hash::Hash for CompileTestResultComparison {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.benchmark.hash(state);
        self.profile.hash(state);
        self.scenario.hash(state);
        self.backend.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeTestResultComparison {
    benchmark: Benchmark,
    comparison: TestResultComparison,
}

impl Deref for RuntimeTestResultComparison {
    type Target = TestResultComparison;

    fn deref(&self) -> &Self::Target {
        &self.comparison
    }
}

impl cmp::PartialEq for RuntimeTestResultComparison {
    fn eq(&self, other: &Self) -> bool {
        self.benchmark == other.benchmark
    }
}

impl cmp::Eq for RuntimeTestResultComparison {}

impl std::hash::Hash for RuntimeTestResultComparison {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.benchmark.hash(state);
    }
}

// The direction of a performance change
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    None,
    Improvement,
    Regression,
    Mixed,
}

// The direction of a performance change. Forms a lattice:
//
//          Mixed
//          /    \
// Improvement  Regression
//          \    /
//           None
//
impl Direction {
    // Also known as the "least upper bound".
    pub fn join(self, other: Self) -> Self {
        match (self, other) {
            (Self::None, b) => b,
            (a, Self::None) => a,
            (Self::Improvement, Self::Improvement) => Self::Improvement,
            (Self::Regression, Self::Regression) => Self::Regression,
            _ => Self::Mixed,
        }
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
}

async fn generate_report(
    start: &Bound,
    end: &Bound,
    summary: String,
    mut report: HashMap<Direction, Vec<String>>,
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
    let regressions = report.remove(&Direction::Regression).unwrap_or_default();
    let improvements = report.remove(&Direction::Improvement).unwrap_or_default();
    let mixed = report.remove(&Direction::Mixed).unwrap_or_default();
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
    let num_regressions = regressions.len();
    let regressions_suffix = if num_regressions == 1 { "" } else { "s" };

    let num_improvements = improvements.len();
    let improvements_suffix = if num_improvements == 1 { "" } else { "s" };

    let first_commit = start;
    let last_commit = end;
    let first_commit_prefix = first_commit.chars().take(8).collect::<String>();
    let last_commit_prefix = last_commit.chars().take(8).collect::<String>();

    let rollup_count = regressions
        .iter()
        .chain(improvements.iter())
        .chain(mixed.iter())
        .filter(|s| s.contains("Rollup of"))
        .count();

    format!(
        r#####"# {date} Triage Log

TODO: Summary

Triage done by **@???**.
Revision range: [{first_commit_prefix}..{last_commit_prefix}](https://perf.rust-lang.org/?start={first_commit}&end={last_commit}&absolute=false&stat=instructions%3Au)

{summary}

{num_regressions} Regression{regressions_suffix}, {num_improvements} Improvement{improvements_suffix}, {num_mixed} Mixed; {rollup_count} of them in rollups
{num_comparisons} artifact comparisons made in total

#### Regressions

{regressions}

#### Improvements

{improvements}

#### Mixed

{mixed}

#### Untriaged Pull Requests

{untriaged}

#### Nags requiring follow up

TODO: Nags

"#####,
        date = chrono::Utc::now().format("%Y-%m-%d"),
        num_comparisons = num_comparisons,
        num_mixed = mixed.len(),
        regressions = regressions.join("\n\n"),
        improvements = improvements.join("\n\n"),
        mixed = mixed.join("\n\n"),
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

    use collector::compile::benchmark::category::Category;

    #[test]
    fn summary_table_only_regressions_primary() {
        check_table(
            vec![
                (Category::Primary, 5.0, 10.0),
                (Category::Primary, 5.0, 12.0),
                (Category::Primary, 1.0, 3.0),
            ],
            r#"
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 146.7% | [100.0%, 200.0%] | 3     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -      | -                | 0     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | 146.7% | [100.0%, 200.0%] | 3     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -      | -                | 0     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -71.7% | [-80.0%, -60.0%] | 3     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | -71.7% | [-80.0%, -60.0%] | 3     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -      | -                | 0     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -      | -                | 0     |
| Improvements ✅ <br /> (secondary) | -71.7% | [-80.0%, -60.0%] | 3     |
| All ❌✅ (primary)                 | -      | -                | 0     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -      | -                | 0     |
| Regressions ❌ <br /> (secondary)  | 146.7% | [100.0%, 200.0%] | 3     |
| Improvements ✅ <br /> (primary)   | -      | -                | 0     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | -      | -                | 0     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 150.0% | [100.0%, 200.0%] | 2     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -62.5% | [-75.0%, -50.0%] | 2     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | 43.8%  | [-75.0%, 200.0%] | 4     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 150.0% | [100.0%, 200.0%] | 2     |
| Regressions ❌ <br /> (secondary)  | 100.0% | [100.0%, 100.0%] | 1     |
| Improvements ✅ <br /> (primary)   | -62.5% | [-75.0%, -50.0%] | 2     |
| Improvements ✅ <br /> (secondary) | -66.7% | [-66.7%, -66.7%] | 1     |
| All ❌✅ (primary)                 | 43.8%  | [-75.0%, 200.0%] | 4     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 20.0%  | [20.0%, 20.0%]   | 1     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -50.0% | [-50.0%, -50.0%] | 1     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | -15.0% | [-50.0%, 20.0%]  | 2     |
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
| (instructions:u)                   | mean   | range            | count |
|:----------------------------------:|:------:|:----------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 100.0% | [100.0%, 100.0%] | 1     |
| Regressions ❌ <br /> (secondary)  | -      | -                | 0     |
| Improvements ✅ <br /> (primary)   | -16.7% | [-16.7%, -16.7%] | 1     |
| Improvements ✅ <br /> (secondary) | -      | -                | 0     |
| All ❌✅ (primary)                 | 41.7%  | [-16.7%, 100.0%] | 2     |
"#
            .trim_start(),
        );
    }

    #[test]
    fn parse_metric_instructions() {
        let metric: Metric = serde_json::from_str(r#""instructions:u""#).unwrap();
        assert!(matches!(metric, Metric::InstructionsUser));
    }

    #[test]
    fn parse_metric_cycles() {
        let metric: Metric = serde_json::from_str(r#""cycles:u""#).unwrap();
        assert!(matches!(metric, Metric::CyclesUser));
    }

    #[test]
    fn parse_metric_max_rss() {
        let metric: Metric = serde_json::from_str(r#""max-rss""#).unwrap();
        assert!(matches!(metric, Metric::MaxRSS));
    }

    // (category, before, after)
    fn check_table(values: Vec<(Category, f64, f64)>, expected: &str) {
        let mut primary_comparisons = Vec::new();
        let mut secondary_comparisons = Vec::new();
        for (category, before, after) in values.into_iter() {
            let target = if category == Category::Primary {
                &mut primary_comparisons
            } else {
                &mut secondary_comparisons
            };

            target.push(TestResultComparison {
                metric: Metric::InstructionsUser,
                historical_data: None,
                results: (before, after),
            });
        }

        let primary = ArtifactComparisonSummary::summarize(primary_comparisons);
        let secondary = ArtifactComparisonSummary::summarize(secondary_comparisons);

        let mut result = String::new();
        write_summary_table(&primary, &secondary, true, &mut result);
        // We don't use `assert_eq!` here because it stringifies the arguments,
        // making the tables hard to read when printed.
        if result != expected {
            panic!(
                "output mismatch:\nexpected:\n{}actual:\n{}",
                expected, result
            );
        }
    }
}
