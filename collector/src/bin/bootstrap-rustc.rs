use jobserver::Client;
use std::env;
use std::process::Command;

fn run() -> i32 {
    let server = Client::new(1).expect("made jobserver");

    let mut cmd = Command::new(env::var_os("RUSTC_PERF_REAL_RUSTC").unwrap());
    cmd.args(env::args_os().skip(1));

    // We want the bootstrap rustc to have access to full parallelism, even
    // though the parent cargo is permitted to spawn at most one rustc (-j1).
    server.configure(&mut cmd);

    let status = cmd.status().expect("spawned");

    if status.success() {
        0
    } else {
        1
    }
}

fn main() {
    std::process::exit(run());
}
