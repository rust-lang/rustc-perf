use futures::stream::{FuturesOrdered, StreamExt};

use crate::api::{bootstrap, ServerResult};
use crate::db::ArtifactId;
use crate::load::SiteCtxt;

pub async fn handle_bootstrap(
    body: bootstrap::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<bootstrap::Response> {
    log::info!("handle_bootstrap({:?})", body);
    let range = ctxt.data_range(body.start.clone()..=body.end.clone());
    let mut commits: Vec<ArtifactId> = range.iter().map(|c| c.clone().into()).collect();

    let conn = ctxt.conn().await;
    let ids = commits
        .iter()
        .map(|c| conn.artifact_id(&c))
        .collect::<FuturesOrdered<_>>()
        .collect::<Vec<_>>()
        .await;
    let by_crate = conn.get_bootstrap(&ids).await;
    let mut by_crate = by_crate
        .into_iter()
        .filter_map(|(k, v)| {
            // We show any line that has at least one point exceeding the
            // critical line.
            if v.iter().any(|v| v.map_or(false, |v| v.as_secs() >= 30)) {
                Some((
                    k,
                    v.into_iter()
                        .map(|v| v.map(|d| d.as_nanos() as u64))
                        .collect(),
                ))
            } else {
                None
            }
        })
        .collect::<hashbrown::HashMap<String, Vec<Option<u64>>>>();

    // Don't return commits/nulls for completely null commits at the beginning
    let start: usize = by_crate
        .values()
        .filter_map(|series| series.iter().position(|v| v.is_some()))
        .min()
        .unwrap_or(0);

    commits = commits.split_off(start);
    for series in by_crate.values_mut() {
        *series = series.split_off(start);
    }

    Ok(bootstrap::Response {
        commits: commits
            .into_iter()
            .map(|v| match v {
                ArtifactId::Commit(c) => (c.date.0.timestamp(), c.sha),
                ArtifactId::Tag(_) => todo!(),
            })
            .collect(),
        by_crate,
    })
}
