use structopt::StructOpt;
use triagebot::{github::GithubClient, handlers::project_goals};

/// A basic example
#[derive(StructOpt, Debug)]
struct Opt {
    /// If specified, no messages are sent.
    #[structopt(long)]
    dry_run: bool,

    /// Goals with an updated within this threshold will not be pinged.
    days_threshold: i64,

    /// A string like "on Sep-5" when the update blog post will be written.
    next_meeting_date: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let opt = Opt::from_args();
    let gh = GithubClient::new_from_env();
    project_goals::ping_project_goals_owners(
        &gh,
        opt.dry_run,
        opt.days_threshold,
        &opt.next_meeting_date,
    )
    .await?;

    Ok(())
}
