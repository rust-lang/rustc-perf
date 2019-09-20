use std::env;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let mut args = env::args_os().skip(1).collect::<Vec<_>>();
    let rustc = env::var_os("RUSTC_REAL").unwrap();

    if let Some(count) = env::var("RUSTC_THREAD_COUNT")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
    {
        args.push(std::ffi::OsString::from(format!("-Zthreads={}", count)));
    }

    if let Some(pos) = args.iter().position(|arg| arg == "--wrap-rustc-with") {
        // Strip out the flag and its argument, and run rustc under the wrapper
        // program named by the argument.
        args.remove(pos);
        let wrapper = args.remove(pos);
        let wrapper = wrapper.to_str().unwrap();

        raise_priority();

        match wrapper {
            "perf-stat" | "perf-stat-self-profile" => {
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

                let prof_out_dir = std::env::current_dir().unwrap().join("self-profile-output");
                if wrapper == "perf-stat-self-profile" {
                    cmd.arg(&format!(
                        "-Zself-profile={}",
                        prof_out_dir.to_str().unwrap()
                    ));
                }

                let start = Instant::now();
                let status = cmd.status().expect("failed to spawn");
                let dur = start.elapsed();
                assert!(status.success());
                print_memory();
                print_time(dur);
                if wrapper == "perf-stat-self-profile" {
                    let crate_name = args
                        .windows(2)
                        .find(|args| args[0] == "--crate-name")
                        .and_then(|args| args[1].to_str())
                        .expect("rustc to be invoked with crate name");
                    let mut prefix = None;
                    // We don't know the pid of rustc, and can't easily get it -- we only know the
                    // `perf` pid. So just blindly look in the directory to hopefully find it.
                    for entry in std::fs::read_dir(&prof_out_dir).unwrap() {
                        let entry = entry.unwrap();
                        if entry
                            .file_name()
                            .to_str()
                            .map_or(false, |s| s.starts_with(crate_name))
                        {
                            let file = entry.file_name().to_str().unwrap().to_owned();
                            prefix = Some(file[..file.find('.').unwrap()].to_owned());
                        }
                    }
                    let prefix = prefix.expect(&format!("found prefix {:?}", prof_out_dir));
                    // FIXME: it'd be great to have a less generic name for this;
                    // we should think about renaming the binary in measureme to measureme, such
                    // that the command is `measureme summarize ...`.
                    let mut cmd = Command::new("summarize");
                    cmd.current_dir(&prof_out_dir);
                    cmd.arg("summarize").arg("--json");
                    cmd.arg(&prefix);
                    let status = cmd.status().expect(&format!(
                        "summarize spawned successfully in {:?}",
                        prof_out_dir
                    ));
                    assert!(
                        status.success(),
                        "summarize failed in {:?}; prefix is {:?}",
                        prof_out_dir,
                        prefix
                    );
                    let json =
                        std::fs::read_to_string(prof_out_dir.join(&format!("{}.json", prefix)))
                            .expect("able to read JSON output");
                    println!("!self-profile-output:{}", json);
                }
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
                cmd.arg("operf").arg(&rustc).args(&args);

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
