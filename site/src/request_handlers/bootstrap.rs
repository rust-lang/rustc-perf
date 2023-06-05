use futures::stream::{FuturesOrdered, StreamExt};

use crate::api::{bootstrap, ServerResult};
use crate::db::ArtifactId;
use crate::load::SiteCtxt;

use std::time::Duration;

pub async fn handle_bootstrap(
    body: bootstrap::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<bootstrap::Response> {
    log::info!("handle_bootstrap({:?})", body);
    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let commits: Vec<ArtifactId> = range
        .into_iter()
        .filter(|c| c.is_master())
        .map(|c| c.into())
        .collect();

    let conn = ctxt.conn().await;
    let ids = commits
        .iter()
        .map(|c| conn.artifact_id(c))
        .collect::<FuturesOrdered<_>>()
        .collect::<Vec<_>>()
        .await;

    let by_crate_build_times = conn.get_bootstrap_by_crate(&ids).await;

    fn duration_as_nanos_u64(d: Duration) -> u64 {
        d.as_nanos() as u64
    }

    let by_crate_build_times = by_crate_build_times
        .into_iter()
        .filter_map(|(k, v)| {
            // We show any line that has at least one point exceeding the
            // critical line.
            if v.iter()
                .any(|v| v.map_or(false, |v| v.as_secs() >= body.min_seconds as u64))
            {
                Some((
                    k,
                    v.into_iter()
                        .map(|v| v.map(duration_as_nanos_u64))
                        .collect(),
                ))
            } else {
                None
            }
        })
        .collect::<hashbrown::HashMap<String, Vec<Option<u64>>>>();

    let total_build_times = conn
        .get_bootstrap(&ids)
        .await
        .into_iter()
        .map(|v| v.map(duration_as_nanos_u64))
        .collect();

    Ok(bootstrap::Response {
        commits: commits
            .into_iter()
            .map(|v| match v {
                ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
                ArtifactId::Tag(_) => todo!(),
            })
            .collect(),
        by_crate_build_times,
        total_build_times,
    })
}
