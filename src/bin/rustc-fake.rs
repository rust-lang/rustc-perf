use std::env;
use std::process::Command;

fn main() {
    let args = env::args_os().skip(1).collect::<Vec<_>>();

    let rustc = env::var_os("RUSTC_REAL").unwrap();
    let mut cmd = Command::new(&rustc);
    let any_time_passes = args.iter().any(|a| a.to_str() == Some("time-passes"));

    if env::var_os("USE_PERF").is_some() && any_time_passes {
        cmd = Command::new("perf");
        cmd.arg("stat")
            .arg("-x;")
            .arg("-e").arg("instructions:u,cycles:u,task-clock,cpu-clock,faults")
            .arg("--log-fd").arg("1")
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

    use std::io;
    use std::mem;

    unsafe {
        // Pin ourselves to just one core in case we're running on a multicore
        // system to try to normalize our results a bit
        let mut set: libc::cpu_set_t = mem::zeroed();
        libc::CPU_ZERO(&mut set);
        libc::CPU_SET(0, &mut set);
        let r = libc::sched_setaffinity(libc::getpid(),
                                        mem::size_of_val(&set),
                                        &set);
        if r != 0 {
            panic!("failed to set affinity: {}", io::Error::last_os_error());
        }

        // Try to reduce jitter in wall time by increasing our priority to the
        // maximum
        let r = libc::setpriority(libc::PRIO_PROCESS as _,
                                  libc::getpid() as libc::id_t,
                                  -20);
        if r != 0 {
            // this requires elevated privileges, so just ignore if this fails,
            // and hope we've got elevated privileges on the actual perf bot.
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
            println!("{},,max-rss,3,100.00", usage.ru_maxrss);
        }
    }
}

#[cfg(windows)]
fn raise_priority() {
}

#[cfg(windows)]
fn print_memory() {
}
