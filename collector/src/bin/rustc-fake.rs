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

        raise_priority();

        match wrapper {
            "perf-stat" => {
                let mut cmd = Command::new("perf");
                let has_perf = cmd.output().is_ok();
                assert!(has_perf);
                cmd.arg("stat")
                    .arg("-x;")
                    .arg("-e")
                    .arg("instructions:u,cycles:u,task-clock,cpu-clock,faults")
                    .arg("--log-fd")
                    .arg("1")
                    .arg(&rustc)
                    .args(&args);

                let start = Instant::now();
                assert!(cmd.status().expect("failed to spawn").success());
                let dur = start.elapsed();
                print_memory();
                print_time(dur);
            }

            "time-passes" => {
                let mut cmd = Command::new(&rustc);
                cmd.arg("-Ztime-passes").args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "perf-record" => {
                let mut cmd = Command::new("perf");
                let has_perf = cmd.output().is_ok();
                assert!(has_perf);
                cmd.arg("record")
                    .arg("--call-graph=dwarf")
                    .arg("--output=perf")
                    .arg("--freq=99")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "oprofile" => {
                let mut cmd = Command::new("operf");
                let has_oprofile = cmd.output().is_ok();
                assert!(has_oprofile);
                // Other possibly useful args: --callgraph, --separate-thread
                cmd.arg("operf")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "cachegrind" => {
                let mut cmd = Command::new("valgrind");
                let has_valgrind = cmd.output().is_ok();
                assert!(has_valgrind);

                // With --cache-sim=no and --branch-sim=no, Cachegrind just
                // collects instruction counts.
                cmd.arg("--tool=cachegrind")
                    .arg("--cache-sim=no")
                    .arg("--branch-sim=no")
                    .arg("--cachegrind-out-file=cgout")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "callgrind" => {
                let mut cmd = Command::new("valgrind");
                let has_valgrind = cmd.output().is_ok();
                assert!(has_valgrind);

                // With --cache-sim=no and --branch-sim=no, Callgrind just
                // collects instruction counts.
                cmd.arg("--tool=callgrind")
                    .arg("--cache-sim=no")
                    .arg("--branch-sim=no")
                    .arg("--callgrind-out-file=clgout")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "exp-dhat" => {
                let mut cmd = Command::new("valgrind");
                let has_valgrind = cmd.output().is_ok();
                assert!(has_valgrind);
                cmd.arg("--tool=exp-dhat")
                    .arg("--show-top-n=500")
                    .arg("--num-callers=4")
                    .arg("--sort-by=tot-blocks-allocd")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "dhat" => {
                let mut cmd = Command::new("valgrind");
                let has_valgrind = cmd.output().is_ok();
                assert!(has_valgrind);
                cmd.arg("--tool=dhat")
                    .arg("--num-callers=4")
                    .arg("--dhat-out-file=dhout")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "massif" => {
                let mut cmd = Command::new("valgrind");
                let has_valgrind = cmd.output().is_ok();
                assert!(has_valgrind);
                cmd.arg("--tool=massif")
                    .arg("--heap-admin=0")
                    .arg("--depth=15")
                    .arg("--threshold=0.2")
                    .arg("--massif-out-file=msout")
                    .arg("--alloc-fn=__rdl_alloc")
                    .arg(&rustc)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "eprintln" => {
                let mut cmd = Command::new(&rustc);
                cmd.args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            _ => {
                panic!("unknown wrapper: {}", wrapper);
            }
        }
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
    use libc;

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
    use libc;

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
