use std::env;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let mut args = env::args_os().skip(1).collect::<Vec<_>>();
    let rustc = env::var_os("RUSTC_REAL").unwrap();

    if let Some(pos) = args.iter().position(|arg| arg == "--wrap-rustc-with") {
        // Strip out the flag and its argument, and run rustc under the wrapper
        // program named by the argument.
        args.remove(pos);
        let wrapper = args.remove(pos);
        let wrapper = wrapper.to_str().unwrap();

        assert!(wrapper == "perf-stat");

        let mut cmd = Command::new("perf");
        cmd.arg("stat")
            .arg("-x;")
            .arg("-e")
            .arg("instructions:u,cycles:u,task-clock,cpu-clock,faults")
            .arg("--log-fd")
            .arg("1")
            .arg(&rustc)
            .args(&args);

        raise_priority();
        let start = Instant::now();
        assert!(cmd.status().expect("failed to spawn").success());
        let dur = start.elapsed();
        print_memory();
        print_time(dur);

    } else {
        let mut cmd = Command::new(&rustc);
        cmd.args(&args);
        exec(&mut cmd);
    }
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> ! {
    use std::os::unix::prelude::*;
    let error = cmd.exec();
    panic!("failed to exec: {}", error);
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
            // for explanation of all the semicolons, see `print_time` below
            println!("{};;max-rss;3;100.00", usage.ru_maxrss);
        }
    }
}

fn print_time(dur: Duration) {
    // Format output the same as `perf stat` in CSV mode, explained at
    // http://man7.org/linux/man-pages/man1/perf-stat.1.html#CSV_FORMAT
    //
    // tl;dr; it's:
    //
    //      $value ; $unit ; $name ; $runtime ; $pct
    println!(
        "{}.{:09};;wall-time;4;100.00",
        dur.as_secs(),
        dur.subsec_nanos()
    );
}

#[cfg(windows)]
fn raise_priority() {}

#[cfg(windows)]
fn print_memory() {}
