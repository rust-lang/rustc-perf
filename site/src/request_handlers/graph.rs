use collector::Bound;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::graph::GraphKind;
use crate::api::{graph, ServerResult};
use crate::db::{self, ArtifactId, Benchmark, Profile, Scenario};
use crate::interpolate::IsInterpolated;
use crate::load::SiteCtxt;
use crate::selector::{Query, Selector, SeriesResponse, Tag};

pub async fn handle_graphs(
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

    let resp = graph_response(body, ctxt).await?;

    if is_default_query {
        ctxt.landing_page.store(Arc::new(Some(resp.clone())));
    }

    Ok(resp)
}

async fn graph_response(
    body: graph::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<Arc<graph::Response>> {
    let range = ctxt.data_range(body.start..=body.end);
    let commits: Arc<Vec<_>> = Arc::new(range.into_iter().map(|c| c.into()).collect());
    let mut benchmarks = HashMap::new();

    let interpolated_responses: Vec<_> = ctxt
        .statistic_series(
            Query::new()
                .set::<String>(Tag::Benchmark, Selector::All)
                .set::<String>(Tag::Profile, Selector::All)
                .set::<String>(Tag::Scenario, Selector::All)
                .set::<String>(Tag::Metric, Selector::One(body.stat)),
            commits.clone(),
        )
        .await?
        .into_iter()
        .map(|sr| sr.interpolate().map(|series| series.collect::<Vec<_>>()))
        .collect();

    let summary_benchmark = create_summary(ctxt, &interpolated_responses, body.kind)?;

    benchmarks.insert("Summary".to_string(), summary_benchmark);

    for response in interpolated_responses {
        let benchmark = response.path.get::<Benchmark>()?.to_string();
        let profile = *response.path.get::<Profile>()?;
        let scenario = response.path.get::<Scenario>()?.to_string();
        let graph_series = graph_series(response.series.into_iter(), body.kind);

        benchmarks
            .entry(benchmark)
            .or_insert_with(HashMap::new)
            .entry(profile)
            .or_insert_with(HashMap::new)
            .insert(scenario, graph_series);
    }

    Ok(Arc::new(graph::Response {
        commits: Arc::try_unwrap(commits)
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

/// Creates a summary "benchmark" that averages the results of all other
/// test cases per profile type
fn create_summary(
    ctxt: &SiteCtxt,
    interpolated_responses: &[SeriesResponse<Vec<((ArtifactId, Option<f64>), IsInterpolated)>>],
    graph_kind: GraphKind,
) -> ServerResult<HashMap<Profile, HashMap<String, graph::Series>>> {
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
) -> graph::Series {
    let mut graph_series = graph::Series {
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
