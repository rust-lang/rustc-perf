use collector::{LocalSelfProfileStorage, S3SelfProfileStorage, SelfProfileStorage};
use futures::future::FutureExt;
use parking_lot::RwLock;
use site::job_queue::create_job_queue_process;
use site::load;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    env_logger::init();

    #[cfg(unix)]
    let _ = jemalloc_ctl::background_thread::write(true);

    let ctxt: Arc<RwLock<Option<Arc<load::SiteCtxt>>>> = Arc::new(RwLock::new(None));
    let ctxt_ = ctxt.clone();
    let db_url = env::var("DATABASE_URL")
        .ok()
        .or_else(|| env::args().nth(1))
        .unwrap_or_else(|| {
            eprintln!("Defaulting to loading from `results.db`");
            String::from("results.db")
        });
    let port = env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(2346);
    let queue_update_interval_seconds = env::var("QUEUE_UPDATE_INTERVAL_SECONDS")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(30);
    let self_profile_storage: Box<dyn SelfProfileStorage + Send + Sync> =
        match env::var("SELF_PROFILE_STORAGE_S3").ok() {
            Some(_) => {
                eprintln!("Loading self-profile data from S3");
                Box::new(S3SelfProfileStorage::new())
            }
            None => {
                eprintln!("Loading self-profile data from local directory");
                Box::new(LocalSelfProfileStorage::default_path())
            }
        };

    let fut = tokio::task::spawn_blocking(move || {
        tokio::task::spawn(async move {
            let res = Arc::new(
                load::SiteCtxt::from_db_url(&db_url, self_profile_storage)
                    .await
                    .unwrap(),
            );
            *ctxt_.write() = Some(res.clone());
            let commits = res.index.load().commits().len();
            let artifacts = res.index.load().artifacts().count();
            if commits + artifacts == 0 {
                eprintln!("Warning: loading complete but no data identified.");
            }
            eprintln!("Loading complete; found {} artifacts", commits + artifacts);
            eprintln!(
                "View the results in a web browser at 'http://localhost:{port}/compare.html'"
            );
        })
    })
    .fuse();
    println!("Starting server with port={port:?}");

    let server = site::server::start(ctxt.clone(), port).fuse();

    let create_job_queue_handler = |ctxt: Arc<RwLock<Option<Arc<load::SiteCtxt>>>>| {
        task::spawn(async move {
            create_job_queue_process(ctxt, Duration::from_secs(queue_update_interval_seconds))
                .await;
        })
        .fuse()
    };

    let mut job_queue_handler = create_job_queue_handler(ctxt.clone());

    futures::pin_mut!(server);
    futures::pin_mut!(fut);
    loop {
        futures::select! {
            _s = server => {
                eprintln!("Server completed unexpectedly.");
                return;
            }
            l = fut => {
                if let Err(e) = l {
                    eprintln!("Loading failed, exiting.");
                    if let Ok(panic) = e.try_into_panic() {
                        std::panic::resume_unwind(panic);
                    }
                }
            }
            // We want to have a panic boundary here; if the job queue handler panics for any
            // reason (e.g. a transient networking/DB issue), we want to continue running it in the
            // future.
            // We thus use tokio::task::spawn, wait for a potential panic, and then restart the
            // task again.
            res = job_queue_handler => {
                // The job queue handler task future has "finished", which means that it had to crash.
                let error = res.expect_err("Job queue handler finished without an error");
                log::error!("The job queue handler has panicked\n{error:?}\nIt will be now restarted");
                job_queue_handler = create_job_queue_handler(ctxt.clone());
            }
        }
    }
}
