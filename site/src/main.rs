use futures::future::FutureExt;
use parking_lot::RwLock;
use site::load;
use std::env;
use std::sync::Arc;

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
    let fut = tokio::task::spawn_blocking(move || {
        tokio::task::spawn(async move {
            let res = Arc::new(load::SiteCtxt::from_db_url(&db_url).await.unwrap());
            *ctxt_.write() = Some(res.clone());
            let commits = res.index.load().commits().len();
            let artifacts = res.index.load().artifacts().count();
            if commits + artifacts == 0 {
                eprintln!("Loading complete but no data identified; exiting.");
                std::process::exit(1);
            }
            eprintln!("Loading complete; found {} artifacts", commits + artifacts);
            eprintln!(
                "View the results in a web browser at 'http://localhost:{port}/compare.html'"
            );
            // Spawn off a task to post the results of any commit results that we
            // are now aware of.
            site::github::post_finished(&res).await;
        })
    })
    .fuse();
    println!("Starting server with port={:?}", port);

    let server = site::server::start(ctxt, port).fuse();
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
        }
    }
}
