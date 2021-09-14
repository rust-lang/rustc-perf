use collector::Bound;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;
use std::sync::Arc;

use crate::api::graph::GraphKind;
use crate::api::{graph, ServerResult};
use crate::db::{self, ArtifactId, Benchmark, Profile, Scenario};
use crate::interpolate::Interpolated;
use crate::load::SiteCtxt;
use crate::selector::{self, PathComponent, Tag};

pub async fn handle_graph_new(
    body: graph::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<graph::NewResponse> {
    log::info!("handle_graph_new({:?})", body);
    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let commits: Vec<ArtifactId> = range.iter().map(|c| c.clone().into()).collect();

    let mut benchmarks = HashMap::new();

    let raw = handle_graph(body, ctxt).await?;

    for (benchmark_, benchmark_data) in raw.benchmarks.iter() {
        let mut by_profile = HashMap::with_capacity(3);

        for (profile, series) in benchmark_data.iter() {
            let mut by_run = HashMap::with_capacity(3);

            for (name, points) in series.iter() {
                let mut series = graph::Series {
                    points: Vec::new(),
                    is_interpolated: Default::default(),
                };

                for (idx, point) in points.iter().enumerate() {
                    series.points.push(point.y);
                    if point.is_interpolated {
                        series.is_interpolated.insert(idx as u16);
                    }
                }

                by_run.insert(name.clone(), series);
            }

            by_profile.insert(profile.parse::<Profile>().unwrap(), by_run);
        }

        benchmarks.insert(benchmark_.clone(), by_profile);
    }

    Ok(graph::NewResponse {
        commits: commits
            .into_iter()
            .map(|c| match c {
                ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
                ArtifactId::Tag(_) => unreachable!(),
            })
            .collect(),
        benchmarks,
    })
}

static INTERPOLATED_COLOR: &str = "#fcb0f1";

pub async fn handle_graph(
    body: graph::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<Arc<graph::Response>> {
    log::info!("handle_graph({:?})", body);
    let is_default_query = body
        == graph::Request {
            start: Bound::None,
            end: Bound::None,
            stat: String::from("instructions:u"),
            kind: graph::GraphKind::Raw,
        };

    if is_default_query {
        match &**ctxt.landing_page.load() {
            Some(resp) => return Ok(resp.clone()),
            None => {}
        }
    }

    let cc = CommitIdxCache::new();
    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let commits: Arc<Vec<_>> = Arc::new(range.iter().map(|c| c.clone().into()).collect());

    let metric_selector = selector::Selector::One(body.stat.clone());

    let series = ctxt
        .statistic_series(
            selector::Query::new()
                .set::<String>(selector::Tag::Benchmark, selector::Selector::All)
                .set::<String>(selector::Tag::Profile, selector::Selector::All)
                .set::<String>(selector::Tag::Scenario, selector::Selector::All)
                .set::<String>(selector::Tag::Metric, metric_selector.clone()),
            commits.clone(),
        )
        .await?;

    let mut series = series
        .into_iter()
        .map(|sr| {
            sr.interpolate()
                .map(|series| to_graph_data(&cc, body.kind, series).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let mut baselines = HashMap::new();
    let c = commits.clone();
    let baselines = &mut baselines;

    let summary_queries = iproduct!(
        ctxt.summary_scenarios(),
        vec![Profile::Check, Profile::Debug, Profile::Opt],
        vec![body.stat.clone()]
    )
    .map(|(scenario, profile, metric)| {
        selector::Query::new()
            .set::<String>(selector::Tag::Benchmark, selector::Selector::All)
            .set(selector::Tag::Profile, selector::Selector::One(profile))
            .set(selector::Tag::Scenario, selector::Selector::One(scenario))
            .set::<String>(selector::Tag::Metric, selector::Selector::One(metric))
    });

    for query in summary_queries {
        let profile = query
            .get(Tag::Profile)
            .unwrap()
            .raw
            .assert_one()
            .parse::<Profile>()
            .unwrap();
        let scenario = query
            .get(Tag::Scenario)
            .unwrap()
            .raw
            .assert_one()
            .parse::<Scenario>()
            .unwrap();
        let q = selector::Query::new()
            .set::<String>(selector::Tag::Benchmark, selector::Selector::All)
            .set(selector::Tag::Profile, selector::Selector::One(profile))
            .set(
                selector::Tag::Scenario,
                selector::Selector::One(Scenario::Empty),
            )
            .set(
                selector::Tag::Metric,
                query.get(Tag::Metric).unwrap().raw.clone(),
            );
        let against = match baselines.entry(q.clone()) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let value = db::average(
                    ctxt.statistic_series(q, c.clone())
                        .await?
                        .into_iter()
                        .map(|sr| sr.interpolate().series)
                        .collect::<Vec<_>>(),
                )
                .next()
                .map_or(0.0, |((_c, d), _interpolated)| d.expect("interpolated"));
                *v.insert(value)
            }
        };
        let averaged = db::average(
            ctxt.statistic_series(query.clone(), commits.clone())
                .await?
                .into_iter()
                .map(|sr| sr.interpolate().series)
                .collect(),
        )
        .map(|((c, d), i)| ((c, Some(d.expect("interpolated") / against)), i));
        let graph_data = to_graph_data(&cc, body.kind, averaged).collect::<Vec<_>>();
        series.push(selector::SeriesResponse {
            path: selector::Path::new()
                .set(PathComponent::Benchmark("Summary".into()))
                .set(PathComponent::Profile(profile))
                .set(PathComponent::Scenario(scenario))
                .set(PathComponent::Metric(
                    query
                        .get(Tag::Metric)
                        .unwrap()
                        .raw
                        .assert_one()
                        .parse()
                        .unwrap(),
                )),
            series: graph_data,
        })
    }

    let mut by_test_case = HashMap::new();
    let mut by_benchmark_max = HashMap::new();
    for sr in series {
        let benchmark = sr.path.get::<Benchmark>()?.to_string();
        let max = by_benchmark_max
            .entry(benchmark.clone())
            .or_insert(f32::MIN);
        *max = sr
            .series
            .iter()
            .map(|p| p.y)
            .fold(*max, |max, p| max.max(p));
        by_test_case
            .entry(benchmark)
            .or_insert_with(HashMap::new)
            .entry(sr.path.get::<Profile>()?.to_string())
            .or_insert_with(Vec::new)
            .push((sr.path.get::<Scenario>()?.to_string(), sr.series));
    }

    let resp = Arc::new(graph::Response {
        max: by_benchmark_max,
        benchmarks: by_test_case,
        colors: vec![String::new(), String::from(INTERPOLATED_COLOR)],
        commits: cc.into_commits(),
    });

    if is_default_query {
        ctxt.landing_page.store(Arc::new(Some(resp.clone())));
    }

    Ok(resp)
}

struct CommitIdxCache {
    commit_idx: RefCell<HashMap<String, u16>>,
    commits: RefCell<Vec<String>>,
}

impl CommitIdxCache {
    fn new() -> Self {
        Self {
            commit_idx: RefCell::new(HashMap::new()),
            commits: RefCell::new(Vec::new()),
        }
    }

    fn into_commits(self) -> Vec<String> {
        std::mem::take(&mut *self.commits.borrow_mut())
    }

    fn lookup(&self, commit: String) -> u16 {
        *self
            .commit_idx
            .borrow_mut()
            .entry(commit.clone())
            .or_insert_with(|| {
                let idx = self.commits.borrow().len();
                self.commits.borrow_mut().push(commit);
                idx.try_into().unwrap_or_else(|_| {
                    panic!("{} too big", idx);
                })
            })
    }
}

fn to_graph_data<'a>(
    cc: &'a CommitIdxCache,
    kind: GraphKind,
    points: impl Iterator<Item = ((ArtifactId, Option<f64>), Interpolated)> + 'a,
) -> impl Iterator<Item = graph::GraphData> + 'a {
    let mut first = None;
    let mut prev = None;
    points.map(move |((aid, point), interpolated)| {
        let commit = if let ArtifactId::Commit(commit) = aid {
            commit
        } else {
            unimplemented!()
        };
        let point = point.expect("interpolated");
        first = Some(first.unwrap_or(point));
        let first = first.unwrap();
        let percent_first = (point - first) / first * 100.0;
        let previous_point = prev.unwrap_or(point);
        let percent_prev = (point - previous_point) / previous_point * 100.0;
        prev = Some(point);
        graph::GraphData {
            commit: cc.lookup(commit.sha),
            absolute: point as f32,
            percent_first: percent_first as f32,
            y: match kind {
                GraphKind::Raw => point as f32,
                GraphKind::PercentRelative => percent_prev as f32,
                GraphKind::PercentFromFirst => percent_first as f32,
            },
            x: commit.date.0.timestamp() as u64 * 1000, // all dates are since 1970
            is_interpolated: interpolated.is_interpolated(),
        }
    })
}
