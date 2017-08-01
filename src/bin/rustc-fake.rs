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
    exec(&mut cmd)
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> ! {
    extern crate libc;

    use std::os::unix::prelude::*;
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

    panic!("failed to spawn: {}", cmd.exec());
}

#[cfg(windows)]
fn exec(cmd: &mut Command) -> ! {
    std::process::exit(cmd.status().expect("failed to spawn").code().unwrap());
}
