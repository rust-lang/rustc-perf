use crate::Commit;
use anyhow::Context as _;

pub fn get_commit_or_fake_it(sha: &str) -> anyhow::Result<Commit> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    Ok(rt
        .block_on(rustc_artifacts::master_commits())
        .map_err(|e| anyhow::anyhow!("{:?}", e))
        .context("getting master commit list")?
        .into_iter()
        .find(|c| c.sha == *sha)
        .map(|c| Commit {
            sha: c.sha.as_str().into(),
            date: c.time.into(),
        })
        .unwrap_or_else(|| {
            log::warn!("utilizing fake commit!");
            Commit {
                sha: sha.into(),
                date: crate::Date::ymd_hms(2000, 01, 01, 0, 0, 0),
            }
        }))
}
