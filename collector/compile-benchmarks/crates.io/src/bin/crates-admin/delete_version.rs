use crate::dialoguer;
use anyhow::Context;
use crates_io::models::update_default_version;
use crates_io::schema::crates;
use crates_io::storage::Storage;
use crates_io::worker::jobs;
use crates_io::{db, schema::versions};
use crates_io_worker::BackgroundJob;
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

#[derive(clap::Parser, Debug)]
#[command(
    name = "delete-version",
    about = "Purge all references to a crate's version from the database.",
    after_help = "Please be super sure you want to do this before running this!"
)]
pub struct Opts {
    /// Name of the crate
    crate_name: String,

    /// Version numbers that should be deleted
    #[arg(value_name = "VERSION", required = true)]
    versions: Vec<String>,

    /// Don't ask for confirmation: yes, we are sure. Best for scripting.
    #[arg(short, long)]
    yes: bool,
}

pub async fn run(opts: Opts) -> anyhow::Result<()> {
    let mut conn = db::oneoff_connection()
        .await
        .context("Failed to establish database connection")?;

    let store = Storage::from_environment();

    let crate_id: i32 = crates::table
        .select(crates::id)
        .filter(crates::name.eq(&opts.crate_name))
        .first(&mut conn)
        .await
        .context("Failed to look up crate id from the database")?;
    {
        let crate_name = &opts.crate_name;

        println!("Deleting the following versions of the `{crate_name}` crate:");
        println!();
        for version in &opts.versions {
            println!(" - {version}");
        }
        println!();

        if !opts.yes
            && !dialoguer::confirm("Do you want to permanently delete these versions?").await?
        {
            return Ok(());
        }
    }

    let opts = conn.transaction(|conn| async move {
        let crate_name = &opts.crate_name;

        info!(%crate_name, %crate_id, versions = ?opts.versions, "Deleting versions from the database");
        let result = diesel::delete(
            versions::table
                .filter(versions::crate_id.eq(crate_id))
                .filter(versions::num.eq_any(&opts.versions)),
        )
        .execute(conn).await;

        match result {
            Ok(num_deleted) if num_deleted == opts.versions.len() => {}
            Ok(num_deleted) => {
                warn!(
                    %crate_name,
                    "Deleted only {num_deleted} of {num_expected} versions from the database",
                    num_expected = opts.versions.len()
                );
            }
            Err(error) => {
                warn!(%crate_name, ?error, "Failed to delete versions from the database")
            }
        }

        info!(%crate_name, %crate_id, "Updating default version in the database");
        if let Err(error) = update_default_version(crate_id, conn).await {
            warn!(%crate_name, %crate_id, ?error, "Failed to update default version");
        }

        Ok::<_, anyhow::Error>(opts)
    }.scope_boxed()).await?;

    let crate_name = &opts.crate_name;

    info!(%crate_name, "Enqueuing index sync jobs");
    let git_index_job = jobs::SyncToGitIndex::new(crate_name);
    let sparse_index_job = jobs::SyncToSparseIndex::new(crate_name);

    if let Err(error) = tokio::try_join!(
        git_index_job.enqueue(&mut conn),
        sparse_index_job.enqueue(&mut conn),
    ) {
        warn!(%crate_name, "Failed to enqueue background job: {error}");
    }

    let mut paths = Vec::new();
    for version in &opts.versions {
        debug!(%crate_name, %version, "Deleting crate file from S3");
        if let Err(error) = store.delete_crate_file(crate_name, version).await {
            warn!(%crate_name, %version, ?error, "Failed to delete crate file from S3");
        } else {
            paths.push(store.crate_location(crate_name, version));
        }

        debug!(%crate_name, %version, "Deleting readme file from S3");
        match store.delete_readme(crate_name, version).await {
            Err(object_store::Error::NotFound { .. }) => {}
            Err(error) => {
                warn!(%crate_name, %version, ?error, "Failed to delete readme file from S3")
            }
            Ok(_) => {
                paths.push(store.readme_location(crate_name, version));
            }
        }
    }

    if let Err(e) = jobs::InvalidateCdns::new(paths.into_iter())
        .enqueue(&mut conn)
        .await
    {
        warn!("{crate_name}: Failed to enqueue CDN invalidation background job: {e}");
    }

    Ok(())
}
