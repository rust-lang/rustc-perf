// Function to match the scheduled job function with its corresponding handler.
// In case you want to add a new one, just add a new clause to the match with
// the job name and the corresponding function.

// Further info could be find in src/jobs.rs

use super::Context;

pub async fn handle_job(
    ctx: &Context,
    name: &str,
    metadata: &serde_json::Value,
) -> anyhow::Result<()> {
    match name {
        "docs_update" => super::docs_update::handle_job().await,
        "rustc_commits" => {
            super::rustc_commits::synchronize_commits_inner(ctx, None).await;
            Ok(())
        }
        _ => default(name, &metadata),
    }
}

fn default(name: &str, metadata: &serde_json::Value) -> anyhow::Result<()> {
    tracing::trace!(
        "handle_job fell into default case: (name={:?}, metadata={:?})",
        name,
        metadata
    );

    Ok(())
}
