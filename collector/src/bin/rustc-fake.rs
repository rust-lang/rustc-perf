use std::env;
use std::process::Command;

fn main() {
    let args = env::args_os().skip(1).collect::<Vec<_>>();

    let rustc = env::var_os("RUSTC_REAL").unwrap();
    let mut cmd = Command::new(&rustc);

    // easiest way to differentiate last pass
    let time_passes = args.iter().position("-Ztime-passes");
    if let Some(pos) = time_passes {
        args.remove(pos);
    }

    if env::var_os("USE_PERF").is_some() && time_passes.is_some() {
        cmd = Command::new("perf");
        cmd.arg("stat")
            .arg("-x;")
            .arg("-e")
            .arg("instructions:u,cycles:u,task-clock,cpu-clock,faults")
            .arg("--log-fd")
            .arg("1")
            .arg(&rustc);
    }
    cmd.args(&args);

    if any_time_passes {
        raise_priority();
        assert!(cmd.status().expect("failed to spawn").success());
        print_memory();
    } else {
        exec(&mut cmd);
    }
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> ! {
    use std::os::unix::prelude::*;
    panic!("failed to spawn: {}", cmd.exec());
}

#[cfg(unix)]
fn raise_priority() {
    extern crate libc;

    unsafe {
        // Try to reduce jitter in wall time by increasing our priority to the
        // maximum
        for i in (1..21).rev() {
            let r = libc::setpriority(libc::PRIO_PROCESS as _, libc::getpid() as libc::id_t, -i);
            if r == 0 {
                break;
            }
        }
    }
}

#[cfg(unix)]
fn print_memory() {
    extern crate libc;

    use std::mem;

    unsafe {
        let mut usage = mem::zeroed();
        let r = libc::getrusage(libc::RUSAGE_CHILDREN, &mut usage);
        if r == 0 {
            println!("{};;max-rss;3;100.00", usage.ru_maxrss);
        }
    }
}

#[cfg(windows)]
fn raise_priority() {}

#[cfg(windows)]
fn print_memory() {}
