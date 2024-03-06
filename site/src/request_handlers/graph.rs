use std::collections::HashMap;
use std::sync::Arc;

use collector::Bound;

use crate::api::detail_sections::CompilationSections;
use crate::api::graphs::GraphKind;
use crate::api::{detail_graphs, detail_sections, graphs, runtime_detail_graphs, ServerResult};
use crate::db::{self, ArtifactId, Profile, Scenario};
use crate::interpolate::IsInterpolated;
use crate::load::SiteCtxt;
use crate::selector::{
    CompileBenchmarkQuery, CompileTestCase, RuntimeBenchmarkQuery, Selector, SeriesResponse,
};
use crate::self_profile::get_or_download_self_profile;

/// Returns data for before/after graphs when comparing a single test result comparison
/// for a compile-time benchmark.
pub async fn handle_compile_detail_graphs(
    request: detail_graphs::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<detail_graphs::Response> {
    log::info!("handle_compile_detail_graphs({:?})", request);

    let artifact_ids = Arc::new(master_artifact_ids_for_range(
        &ctxt,
        request.start,
        request.end,
    ));

    let scenario = request.scenario.parse()?;
    let interpolated_responses: Vec<_> = ctxt
        .statistic_series(
            CompileBenchmarkQuery::default()
                .benchmark(Selector::One(request.benchmark.clone()))
                .profile(Selector::One(request.profile.parse()?))
                .scenario(Selector::One(scenario))
                .metric(Selector::One(request.stat.parse()?)),
            artifact_ids.clone(),
        )
        .await?
        .into_iter()
        .map(|sr| sr.interpolate().map(|series| series.collect::<Vec<_>>()))
        .collect();

    let mut graphs = Vec::new();

    let mut interpolated_responses = interpolated_responses.into_iter();
    if let Some(response) = interpolated_responses.next() {
        let series = response.series.into_iter().collect::<Vec<_>>();
        for kind in request.kinds {
            graphs.push(graph_series(series.clone().into_iter(), kind));
        }
    }
    assert!(interpolated_responses.next().is_none());

    Ok(detail_graphs::Response {
        commits: artifact_ids_to_commits(artifact_ids),
        graphs,
    })
}

/// Returns data for compilation sections (frontend/backend/linker) when comparing a single test
/// result comparison for a compile-time benchmark.
pub async fn handle_compile_detail_sections(
    request: detail_sections::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<detail_sections::Response> {
    log::info!("handle_compile_detail_sections({:?})", request);

    let start_artifact = ctxt
        .artifact_id_for_bound(request.start.clone(), true)
        .ok_or(format!(
            "could not find start commit for bound {:?}",
            request.start
        ))?;
    let end_artifact = ctxt
        .artifact_id_for_bound(request.end.clone(), false)
        .ok_or(format!(
            "could not find end commit for bound {:?}",
            request.end
        ))?;

    let scenario = request.scenario.parse()?;

    async fn calculate_sections(
        ctxt: &SiteCtxt,
        aid: ArtifactId,
        benchmark: &str,
        profile: &str,
        scenario: Scenario,
    ) -> Option<CompilationSections> {
        get_or_download_self_profile(ctxt, aid, benchmark, profile, scenario, None)
            .await
            .ok()
            .map(|profile| CompilationSections {
                sections: profile.compilation_sections,
            })
    }

    // Doc queries are not split into the classic frontend/backend/linker parts.
    let (before, after) = if request.profile != "doc" {
        tokio::join!(
            calculate_sections(
                &ctxt,
                start_artifact,
                &request.benchmark,
                &request.profile,
                scenario,
            ),
            calculate_sections(
                &ctxt,
                end_artifact,
                &request.benchmark,
                &request.profile,
                scenario,
            )
        )
    } else {
        (None, None)
    };

    Ok(detail_sections::Response { before, after })
}

pub async fn handle_runtime_detail_graphs(
    request: runtime_detail_graphs::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<runtime_detail_graphs::Response> {
    log::info!("handle_runtime_detail_graphs({:?})", request);

    let artifact_ids = Arc::new(master_artifact_ids_for_range(
        &ctxt,
        request.start,
        request.end,
    ));

    let interpolated_responses: Vec<_> = ctxt
        .statistic_series(
            RuntimeBenchmarkQuery::default()
                .benchmark(Selector::One(request.benchmark.clone()))
                .metric(Selector::One(request.stat.parse()?)),
            artifact_ids.clone(),
        )
        .await?
        .into_iter()
        .map(|sr| sr.interpolate().map(|series| series.collect::<Vec<_>>()))
        .collect();

    let mut graphs = Vec::new();

    let mut interpolated_responses = interpolated_responses.into_iter();
    if let Some(response) = interpolated_responses.next() {
        let series = response.series.into_iter().collect::<Vec<_>>();
        for kind in request.kinds {
            graphs.push(graph_series(series.clone().into_iter(), kind));
        }
    }
    assert!(interpolated_responses.next().is_none());

    Ok(runtime_detail_graphs::Response {
        commits: artifact_ids_to_commits(artifact_ids),
        graphs,
    })
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

    let resp = Arc::new(create_graphs(request, &ctxt).await?);

    if is_default_query {
        ctxt.landing_page.store(Arc::new(Some(resp.clone())));
    }

    Ok(resp)
}

async fn create_graphs(
    request: graphs::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<graphs::Response> {
    let artifact_ids = Arc::new(master_artifact_ids_for_range(
        ctxt,
        request.start,
        request.end,
    ));
    let mut benchmarks = HashMap::new();

    let create_selector = |filter: &Option<String>| -> Selector<String> {
        filter
            .as_ref()
            .map(|value| Selector::One(value.clone()))
            .unwrap_or(Selector::All)
    };

    let benchmark_selector = create_selector(&request.benchmark);
    let profile_selector = create_selector(&request.profile).try_map(|v| v.parse::<Profile>())?;
    let scenario_selector =
        create_selector(&request.scenario).try_map(|v| v.parse::<Scenario>())?;

    let interpolated_responses: Vec<_> = ctxt
        .statistic_series(
            CompileBenchmarkQuery::default()
                .benchmark(benchmark_selector)
                .profile(profile_selector)
                .scenario(scenario_selector)
                .metric(Selector::One(request.stat.parse()?)),
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
        let benchmark = response.test_case.benchmark.to_string();
        let profile = response.test_case.profile;
        let scenario = response.test_case.scenario.to_string();
        let graph_series = graph_series(response.series.into_iter(), request.kind);

        benchmarks
            .entry(benchmark)
            .or_insert_with(HashMap::new)
            .entry(profile)
            .or_insert_with(HashMap::new)
            .insert(scenario, graph_series);
    }

    Ok(graphs::Response {
        commits: artifact_ids_to_commits(artifact_ids),
        benchmarks,
    })
}

fn artifact_ids_to_commits(artifact_ids: Arc<Vec<ArtifactId>>) -> Vec<(i64, String)> {
    Arc::try_unwrap(artifact_ids)
        .unwrap()
        .into_iter()
        .map(|c| match c {
            ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
            ArtifactId::Tag(_) => unreachable!(),
        })
        .collect()
}

/// Returns master commit artifact IDs for the given range.
fn master_artifact_ids_for_range(ctxt: &SiteCtxt, start: Bound, end: Bound) -> Vec<ArtifactId> {
    ctxt.data_range(start..=end)
        .into_iter()
        .filter(|commit| commit.is_master())
        .map(|commit| commit.into())
        .collect()
}

#[allow(clippy::type_complexity)]
/// Creates a summary "benchmark" that averages the results of all other
/// test cases per profile type
fn create_summary(
    ctxt: &SiteCtxt,
    interpolated_responses: &[SeriesResponse<
        CompileTestCase,
        Vec<((ArtifactId, Option<f64>), IsInterpolated)>,
    >],
    graph_kind: GraphKind,
) -> ServerResult<HashMap<Profile, HashMap<String, graphs::Series>>> {
    let mut baselines = HashMap::new();
    let mut summary_benchmark = HashMap::new();
    let summary_query_cases = iproduct!(
        ctxt.summary_scenarios(),
        vec![Profile::Check, Profile::Debug, Profile::Opt, Profile::Doc]
    );
    for (scenario, profile) in summary_query_cases {
        let baseline = match baselines.entry((profile, scenario)) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let baseline_responses = interpolated_responses
                    .iter()
                    .filter(|sr| {
                        let p = sr.test_case.profile;
                        let s = sr.test_case.scenario;
                        p == profile && s == Scenario::Empty
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
                let p = sr.test_case.profile;
                let s = sr.test_case.scenario;
                p == profile && s == scenario
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
