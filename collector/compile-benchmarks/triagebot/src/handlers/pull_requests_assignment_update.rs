use crate::handlers::pr_tracking::load_workqueue;
use crate::jobs::Job;
use async_trait::async_trait;

pub struct PullRequestAssignmentUpdate;

#[async_trait]
impl Job for PullRequestAssignmentUpdate {
    fn name(&self) -> &'static str {
        "pull_request_assignment_update"
    }

    async fn run(&self, ctx: &super::Context, _metadata: &serde_json::Value) -> anyhow::Result<()> {
        tracing::trace!("starting pull_request_assignment_update");
        let workqueue = load_workqueue(&ctx.octocrab).await?;
        *ctx.workqueue.write().await = workqueue;
        tracing::trace!("finished pull_request_assignment_update");

        Ok(())
    }
}
