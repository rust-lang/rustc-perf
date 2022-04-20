use collector::Bound;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::graphs::GraphKind;
use crate::api::{graph, graphs, ServerResult};
use crate::db::{self, ArtifactId, Benchmark, Profile, Scenario};
use crate::interpolate::IsInterpolated;
use crate::load::SiteCtxt;
use crate::selector::{Query, Selector, SeriesResponse, Tag};

pub async fn handle_graph(
    request: graph::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<graph::Response> {
    log::info!("handle_graph({:?})", request);

    create_graph(request, ctxt).await
}

pub async fn handle_graphs(
    request: graphs::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<Arc<graphs::Response>> {
    log::info!("handle_graphs({:?})", request);

    let is_default_query = request
        == graphs::Request {
            start: Bound::None,
            end: Bound::None,
            stat: String::from("instructions:u"),
            kind: graphs::GraphKind::Raw,
            benchmark: None,
            scenario: None,
            profile: None,
        };

    if is_default_query {
        match &**ctxt.landing_page.load() {
            Some(resp) => return Ok(resp.clone()),
            None => {}
        }
    }

    let resp = create_graphs(request, &ctxt).await?;

    if is_default_query {
        ctxt.landing_page.store(Arc::new(Some(resp.clone())));
    }

    Ok(resp)
}

async fn create_graph(
    request: graph::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<graph::Response> {
    let artifact_ids = artifact_ids_for_range(&ctxt, request.start, request.end);
    let mut series_iterator = ctxt
        .statistic_series(
            Query::new()
                .set::<String>(Tag::Benchmark, Selector::One(request.benchmark))
                .set::<String>(Tag::Profile, Selector::One(request.profile))
                .set::<String>(Tag::Scenario, Selector::One(request.scenario))
                .set::<String>(Tag::Metric, Selector::One(request.metric)),
            Arc::new(artifact_ids),
        )
        .await?
        .into_iter()
        .map(SeriesResponse::interpolate);

    let result = series_iterator.next().unwrap();
    let graph_series = graph_series(result.series, request.kind);
    Ok(graph::Response {
        series: graph_series,
    })
}

async fn create_graphs(
    request: graphs::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<Arc<graphs::Response>> {
    let artifact_ids = Arc::new(artifact_ids_for_range(ctxt, request.start, request.end));
    let mut benchmarks = HashMap::new();

    let create_selector = |filter: &Option<String>| -> Selector<String> {
        filter
            .as_ref()
            .map(|value| Selector::One(value.clone()))
            .unwrap_or(Selector::All)
    };

    let benchmark_selector = create_selector(&request.benchmark);
    let profile_selector = create_selector(&request.profile);
    let scenario_selector = create_selector(&request.scenario);

    let interpolated_responses: Vec<_> = ctxt
        .statistic_series(
            Query::new()
                .set::<String>(Tag::Benchmark, benchmark_selector)
                .set::<String>(Tag::Profile, profile_selector)
                .set::<String>(Tag::Scenario, scenario_selector)
                .set::<String>(Tag::Metric, Selector::One(request.stat)),
            artifact_ids.clone(),
        )
        .await?
        .into_iter()
        .map(|sr| sr.interpolate().map(|series| series.collect::<Vec<_>>()))
        .collect();

    if request.benchmark.is_none() {
        let summary_benchmark = create_summary(ctxt, &interpolated_responses, request.kind)?;
        benchmarks.insert("Summary".to_string(), summary_benchmark);
    }

    for response in interpolated_responses {
        let benchmark = response.path.get::<Benchmark>()?.to_string();
        let profile = *response.path.get::<Profile>()?;
        let scenario = response.path.get::<Scenario>()?.to_string();
        let graph_series = graph_series(response.series.into_iter(), request.kind);

        benchmarks
            .entry(benchmark)
            .or_insert_with(HashMap::new)
            .entry(profile)
            .or_insert_with(HashMap::new)
            .insert(scenario, graph_series);
    }

    Ok(Arc::new(graphs::Response {
        commits: Arc::try_unwrap(artifact_ids)
            .unwrap()
            .into_iter()
            .map(|c| match c {
                ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
                ArtifactId::Tag(_) => unreachable!(),
            })
            .collect(),
        benchmarks,
    }))
}

fn artifact_ids_for_range(ctxt: &SiteCtxt, start: Bound, end: Bound) -> Vec<ArtifactId> {
    let range = ctxt.data_range(start..=end);
    range.into_iter().map(|c| c.into()).collect()
}

/// Creates a summary "benchmark" that averages the results of all other
/// test cases per profile type
fn create_summary(
    ctxt: &SiteCtxt,
    interpolated_responses: &[SeriesResponse<Vec<((ArtifactId, Option<f64>), IsInterpolated)>>],
    graph_kind: GraphKind,
) -> ServerResult<HashMap<Profile, HashMap<String, graphs::Series>>> {
    let mut baselines = HashMap::new();
    let mut summary_benchmark = HashMap::new();
    let summary_query_cases = iproduct!(
        ctxt.summary_scenarios(),
        vec![Profile::Check, Profile::Debug, Profile::Opt]
    );
    for (scenario, profile) in summary_query_cases {
        let baseline = match baselines.entry((profile, scenario)) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let baseline_responses = interpolated_responses
                    .iter()
                    .filter(|sr| {
                        let p = sr.path.get::<Profile>().unwrap();
                        let s = sr.path.get::<Scenario>().unwrap();
                        *p == profile && *s == Scenario::Empty
                    })
                    .map(|sr| sr.series.iter().cloned())
                    .collect();

                let value = db::average(baseline_responses)
                    .next()
                    .map_or(0.0, |((_c, d), _interpolated)| d.expect("interpolated"));
                *v.insert(value)
            }
        };

        let summary_case_responses = interpolated_responses
            .iter()
            .filter(|sr| {
                let p = sr.path.get::<Profile>().unwrap();
                let s = sr.path.get::<Scenario>().unwrap();
                *p == profile && *s == scenario
            })
            .map(|sr| sr.series.iter().cloned())
            .collect();

        let avg_vs_baseline = db::average(summary_case_responses)
            .map(|((c, d), i)| ((c, Some(d.expect("interpolated") / baseline)), i));

        let graph_series = graph_series(avg_vs_baseline, graph_kind);

        summary_benchmark
            .entry(profile)
            .or_insert_with(HashMap::new)
            .insert(scenario.to_string(), graph_series);
    }
    Ok(summary_benchmark)
}

fn graph_series(
    points: impl Iterator<Item = ((ArtifactId, Option<f64>), IsInterpolated)>,
    kind: GraphKind,
) -> graphs::Series {
    let mut graph_series = graphs::Series {
        points: Vec::new(),
        interpolated_indices: Default::default(),
    };

    let mut first = None;
    let mut prev = None;

    for (idx, ((_aid, point), is_interpolated)) in points.enumerate() {
        let point = point.expect("interpolated point still produced an empty value");
        first = Some(first.unwrap_or(point));
        let first = first.unwrap();
        let percent_first = (point - first) / first * 100.0;
        let previous_point = prev.unwrap_or(point);
        let percent_prev = (point - previous_point) / previous_point * 100.0;
        prev = Some(point);

        let value = match kind {
            GraphKind::Raw => point,
            GraphKind::PercentRelative => percent_prev,
            GraphKind::PercentFromFirst => percent_first,
        } as f32;

        graph_series.points.push(value);

        if is_interpolated.as_bool() {
            graph_series.interpolated_indices.insert(idx as u16);
        }
    }

    graph_series
}
