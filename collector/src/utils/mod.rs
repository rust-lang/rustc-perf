use std::future::Future;

pub mod fs;
pub mod git;
pub mod read2;

pub fn wait_for_future<F: Future<Output = R>, R>(f: F) -> R {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}
