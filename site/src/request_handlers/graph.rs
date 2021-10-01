use collector::Bound;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::graph::GraphKind;
use crate::api::{graph, ServerResult};
use crate::db::{self, ArtifactId, Benchmark, Profile, Scenario};
use crate::interpolate::Interpolated;
use crate::load::SiteCtxt;
use crate::selector::{Path, PathComponent, Query, Selector, SeriesResponse, Tag};

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

    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let commits: Vec<ArtifactId> = range.iter().map(|c| c.clone().into()).collect();

    let mut benchmarks = HashMap::new();

    let benchmarks_impl = handle_graph_impl(body, ctxt).await?;

    for (benchmark_, benchmark_data) in benchmarks_impl.iter() {
        let mut by_profile = HashMap::with_capacity(3);

        for (profile, series) in benchmark_data.iter() {
            let mut by_run = HashMap::with_capacity(3);

            for (name, points) in series.iter() {
                let mut series = graph::Series {
                    points: Vec::new(),
                    is_interpolated: Default::default(),
                };

                for (idx, point) in points.iter().enumerate() {
                    series.points.push(point.value);
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

    let resp = Arc::new(graph::Response {
        commits: commits
            .into_iter()
            .map(|c| match c {
                ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
                ArtifactId::Tag(_) => unreachable!(),
            })
            .collect(),
        benchmarks,
    });

    if is_default_query {
        ctxt.landing_page.store(Arc::new(Some(resp.clone())));
    }

    Ok(resp)
}

struct GraphPoint {
    value: f32,
    is_interpolated: bool,
}

async fn handle_graph_impl(
    body: graph::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<HashMap<String, HashMap<String, Vec<(String, Vec<GraphPoint>)>>>> {
    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let commits: Arc<Vec<_>> = Arc::new(range.iter().map(|c| c.clone().into()).collect());

    let metric: database::Metric = body.stat.parse().unwrap();
    let metric_selector = Selector::One(body.stat.clone());

    let series = ctxt
        .statistic_series(
            Query::new()
                .set::<String>(Tag::Benchmark, Selector::All)
                .set::<String>(Tag::Profile, Selector::All)
                .set::<String>(Tag::Scenario, Selector::All)
                .set::<String>(Tag::Metric, metric_selector.clone()),
            commits.clone(),
        )
        .await?;

    let mut series = series
        .into_iter()
        .map(|sr| {
            sr.interpolate()
                .map(|series| to_graph_points(body.kind, series).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let mut baselines = HashMap::new();

    let summary_query_cases = iproduct!(
        ctxt.summary_scenarios(),
        vec![Profile::Check, Profile::Debug, Profile::Opt]
    );

    for (scenario, profile) in summary_query_cases {
        let query = Query::new()
            .set::<String>(Tag::Benchmark, Selector::All)
            .set(Tag::Profile, Selector::One(profile))
            .set(Tag::Scenario, Selector::One(scenario))
            .set(Tag::Metric, metric_selector.clone());

        let baseline_query = Query::new()
            .set::<String>(Tag::Benchmark, Selector::All)
            .set(Tag::Profile, Selector::One(profile))
            .set(Tag::Scenario, Selector::One(Scenario::Empty))
            .set(Tag::Metric, metric_selector.clone());

        let baseline = match baselines.entry(baseline_query.clone()) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let value = db::average(
                    ctxt.statistic_series(baseline_query, commits.clone())
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

        let avg_vs_baseline = db::average(
            ctxt.statistic_series(query.clone(), commits.clone())
                .await?
                .into_iter()
                .map(|sr| sr.interpolate().series)
                .collect(),
        )
        .map(|((c, d), i)| ((c, Some(d.expect("interpolated") / baseline)), i));

        let graph_data = to_graph_points(body.kind, avg_vs_baseline).collect::<Vec<_>>();

        series.push(SeriesResponse {
            path: Path::new()
                .set(PathComponent::Benchmark("Summary".into()))
                .set(PathComponent::Profile(profile))
                .set(PathComponent::Scenario(scenario))
                .set(PathComponent::Metric(metric)),
            series: graph_data,
        })
    }

    let mut by_test_case = HashMap::new();
    for sr in series {
        let benchmark = sr.path.get::<Benchmark>()?.to_string();
        by_test_case
            .entry(benchmark)
            .or_insert_with(HashMap::new)
            .entry(sr.path.get::<Profile>()?.to_string())
            .or_insert_with(Vec::new)
            .push((sr.path.get::<Scenario>()?.to_string(), sr.series));
    }

    Ok(by_test_case)
}

fn to_graph_points<'a>(
    kind: GraphKind,
    points: impl Iterator<Item = ((ArtifactId, Option<f64>), Interpolated)> + 'a,
) -> impl Iterator<Item = GraphPoint> + 'a {
    let mut first = None;
    let mut prev = None;
    points.map(move |((_aid, point), interpolated)| {
        let point = point.expect("interpolated");
        first = Some(first.unwrap_or(point));
        let first = first.unwrap();
        let percent_first = (point - first) / first * 100.0;
        let previous_point = prev.unwrap_or(point);
        let percent_prev = (point - previous_point) / previous_point * 100.0;
        prev = Some(point);
        GraphPoint {
            value: match kind {
                GraphKind::Raw => point as f32,
                GraphKind::PercentRelative => percent_prev as f32,
                GraphKind::PercentFromFirst => percent_first as f32,
            },
            is_interpolated: interpolated.is_interpolated(),
        }
    })
}
