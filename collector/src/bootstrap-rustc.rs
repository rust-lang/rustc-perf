use jobserver::Client;
use std::env;
use std::process::Command;

fn run() -> i32 {
    let server = Client::new(num_cpus::get()).expect("made jobserver");

    let mut cmd = Command::new(env::var_os("RUSTC_PERF_REAL_RUSTC").unwrap());
    cmd.args(env::args_os());

    // We want the bootstrap rustc to have access to full parallelism, even
    // though the parent cargo is permitted to spawn at most one rustc (-j1).
    server.configure(&mut cmd);

    let status = cmd.status().expect("spawned");

    if status.success() {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    std::process::exit(run());
}
