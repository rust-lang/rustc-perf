use std::sync::Arc;

use lazy_static::lazy_static;

use crate::api::{dashboard, ServerResult};
use crate::benchmark_metadata::get_stable_benchmark_names;
use crate::comparison::Metric;
use crate::db::{self, ArtifactId, Profile, Scenario};
use crate::load::SiteCtxt;
use crate::selector;

pub async fn handle_dashboard(ctxt: Arc<SiteCtxt>) -> ServerResult<dashboard::Response> {
    let index = ctxt.index.load();
    if index.artifacts().next().is_none() {
        return Ok(dashboard::Response::default());
    }

    let mut versions = index
        .artifacts()
        .filter(|a| a.starts_with("1.") || a.starts_with("beta"))
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

    lazy_static! {
        static ref STABLE_BENCHMARKS: Vec<String> = get_stable_benchmark_names();
    }

    let query = selector::CompileBenchmarkQuery::default()
        .benchmark(selector::Selector::Subset(STABLE_BENCHMARKS.clone()))
        .metric(selector::Selector::One(Metric::WallTime));

    let summary_scenarios = ctxt.summary_scenarios();
    let by_profile = ByProfile::new::<String, _, _>(|profile| {
        let summary_scenarios = &summary_scenarios;
        let ctxt = &ctxt;
        let query = &query;
        let aids = &artifact_ids;
        async move {
            let mut cases = dashboard::Cases::default();
            for scenario in summary_scenarios.iter() {
                let responses = ctxt
                    .statistic_series(
                        query
                            .clone()
                            .profile(selector::Selector::One(profile))
                            .scenario(selector::Selector::One(*scenario)),
                        aids.clone(),
                    )
                    .await?;

                let points = db::average(
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
    })
}

pub struct ByProfile<T> {
    pub check: T,
    pub debug: T,
    pub doc: T,
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
            Profile::Opt => &self.opt,
            Profile::Clippy => &self.clippy,
        }
    }
}
