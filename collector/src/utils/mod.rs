use std::future::Future;
use std::process::Command;

pub mod cachegrind;
pub mod fs;
pub mod git;
pub mod mangling;
pub mod read2;

pub fn wait_for_future<F: Future<Output = R>, R>(f: F) -> R {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}

/// Checks if the given binary can be executed.
pub fn is_installed(name: &str) -> bool {
    Command::new(name).output().is_ok()
}
