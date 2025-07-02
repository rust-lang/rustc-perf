use std::sync::{Arc, LazyLock};

use crate::api::{dashboard, ServerResult};
use crate::benchmark_metadata::get_stable_benchmark_names;
use crate::load::SiteCtxt;
use database::selector;
use database::{self, metric::Metric, ArtifactId, Profile, Scenario};

pub async fn handle_dashboard(ctxt: Arc<SiteCtxt>) -> ServerResult<dashboard::Response> {
    let index = ctxt.index.load();
    if index.artifacts().next().is_none() {
        return Ok(dashboard::Response::default());
    }

    let mut versions = index
        .artifacts()
        // Do not consider patch releases, only consider 1.XYZ.0
        .filter(|a| (a.starts_with("1.") && a.ends_with(".0")) || a.starts_with("beta"))
        .collect::<Vec<_>>();
    versions.sort_by(|a, b| {
        match (
            a.parse::<semver::Version>().ok(),
            b.parse::<semver::Version>().ok(),
        ) {
            (Some(a), Some(b)) => a.cmp(&b),
            (_, _) => {
                use std::cmp::Ordering;

                if a.starts_with("beta-") && b.starts_with("beta-") {
                    let a_date = a
                        .strip_prefix("beta-")
                        .unwrap()
                        .parse::<chrono::NaiveDate>();
                    let b_date = b
                        .strip_prefix("beta-")
                        .unwrap()
                        .parse::<chrono::NaiveDate>();
                    if let (Some(a), Some(b)) = (a_date.ok(), b_date.ok()) {
                        a.cmp(&b)
                    } else {
                        log::error!(
                            "Parse failed: {:?} => {:?}, {:?} => {:?}",
                            a,
                            a_date,
                            b,
                            b_date
                        );
                        Ordering::Equal
                    }
                } else if a.starts_with("beta") {
                    Ordering::Greater
                } else if b.starts_with("beta") {
                    Ordering::Less
                } else {
                    // These are both local ids, not a commit.
                    // There's no way to tell which version they are, so just pretend they're the same.
                    Ordering::Equal
                }
            }
        }
    });
    let first_beta = versions.iter().position(|v| v.starts_with("beta-"));
    if let Some(first_beta) = first_beta {
        let last_beta = versions
            .iter()
            .rposition(|v| v.starts_with("beta-"))
            .unwrap();
        // Remove all but the latest beta version, which is the most recent.
        versions.drain(first_beta..last_beta);
    }

    let artifact_ids = Arc::new(
        versions
            .into_iter()
            .map(|v| ArtifactId::Tag(v.to_string()))
            .collect::<Vec<_>>(),
    );

    static STABLE_BENCHMARKS: LazyLock<Vec<String>> = LazyLock::new(get_stable_benchmark_names);

    let compile_benchmark_query = selector::CompileBenchmarkQuery::default()
        .benchmark(selector::Selector::Subset(STABLE_BENCHMARKS.clone()))
        .metric(selector::Selector::One(Metric::WallTime));

    let summary_scenarios = ctxt.summary_scenarios();
    let aids = &artifact_ids;
    let by_profile = ByProfile::new::<String, _, _>(|profile| {
        let summary_scenarios = &summary_scenarios;
        let ctxt = &ctxt;
        let compile_benchmark_query = &compile_benchmark_query;
        async move {
            let mut cases = dashboard::Cases::default();
            for scenario in summary_scenarios.iter() {
                let responses = ctxt
                    .statistic_series(
                        compile_benchmark_query
                            .clone()
                            .profile(selector::Selector::One(profile))
                            .scenario(selector::Selector::One(*scenario)),
                        aids.clone(),
                    )
                    .await?;

                let points = crate::average::average(
                    responses
                        .into_iter()
                        .map(|sr| sr.interpolate().series)
                        .collect::<Vec<_>>(),
                )
                .map(|((_id, point), _interpolated)| {
                    (point.expect("interpolated") * 100.0).round() / 100.0
                })
                .collect::<Vec<_>>();

                match scenario {
                    Scenario::Empty => cases.clean_averages = points,
                    Scenario::IncrementalEmpty => cases.base_incr_averages = points,
                    Scenario::IncrementalFresh => cases.clean_incr_averages = points,
                    // we only have println patches here
                    Scenario::IncrementalPatch(_) => cases.println_incr_averages = points,
                }
            }
            Ok(cases)
        }
    })
    .await
    .unwrap();

    let runtime_benchmark_query = selector::RuntimeBenchmarkQuery::default()
        .benchmark(selector::Selector::All)
        .metric(selector::Selector::One(Metric::WallTime));

    let responses = ctxt
        .statistic_series(runtime_benchmark_query.clone(), aids.clone())
        .await?;

    // The flag is used to ignore only the initial values where the runtime benchmark was not implemented.
    let mut ignore_runtime_benchmark = true;
    let points = crate::average::average(
        responses
            .into_iter()
            .map(|sr| sr.interpolate().series)
            .collect::<Vec<_>>(),
    )
    .map(|((_id, point), interpolated)| {
        if !interpolated.as_bool() {
            ignore_runtime_benchmark = false;
        }

        if ignore_runtime_benchmark && interpolated.as_bool() {
            None
        } else {
            Some((point.expect("interpolated") * 100.0).round() / 100.0)
        }
    })
    .collect::<Vec<_>>();

    Ok(dashboard::Response {
        versions: artifact_ids
            .iter()
            .map(|aid| match aid {
                ArtifactId::Commit(c) => format!("master: {}", &c.sha.to_string()[0..8]),
                ArtifactId::Tag(aid) => aid.clone(),
            })
            .collect::<Vec<_>>(),
        check: by_profile.check,
        debug: by_profile.debug,
        opt: by_profile.opt,
        doc: by_profile.doc,
        runtime: points,
    })
}

pub struct ByProfile<T> {
    pub check: T,
    pub debug: T,
    pub doc: T,
    pub doc_json: T,
    pub opt: T,
    pub clippy: T,
}

impl<T> ByProfile<T> {
    pub async fn new<E, F, F1>(mut f: F) -> Result<Self, E>
    where
        F: FnMut(Profile) -> F1,
        F1: std::future::Future<Output = Result<T, E>>,
    {
        Ok(ByProfile {
            check: f(Profile::Check).await?,
            debug: f(Profile::Debug).await?,
            doc: f(Profile::Doc).await?,
            doc_json: f(Profile::DocJson).await?,
            opt: f(Profile::Opt).await?,
            clippy: f(Profile::Clippy).await?,
        })
    }
}

impl<T> std::ops::Index<Profile> for ByProfile<T> {
    type Output = T;
    fn index(&self, index: Profile) -> &Self::Output {
        match index {
            Profile::Check => &self.check,
            Profile::Debug => &self.debug,
            Profile::Doc => &self.doc,
            Profile::DocJson => &self.doc_json,
            Profile::Opt => &self.opt,
            Profile::Clippy => &self.clippy,
        }
    }
}
