use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let mut args_os = env::args_os();
    let name = args_os.next().unwrap().into_string().unwrap();

    let mut args = args_os.collect::<Vec<_>>();
    let rustc = env::var_os("RUSTC_REAL").unwrap();
    let actually_rustdoc = name.ends_with("rustdoc-fake");
    let tool = if actually_rustdoc {
        let rustdoc = env::var_os("RUSTDOC_REAL").unwrap();
        rustdoc
    } else {
        rustc
    };

    if let Some(count) = env::var("RUSTC_THREAD_COUNT")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
    {
        args.push(OsString::from(format!("-Zthreads={}", count)));
    }

    args.push(OsString::from("-Adeprecated"));
    args.push(OsString::from("-Aunknown-lints"));

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
                    // perf respects this environment variable for e.g., percents in
                    // the output, but we want standard output on all systems.
                    // See #753 for more details.
                    .env("LC_NUMERIC", "C")
                    .arg("-x;")
                    .arg("-e")
                    .arg("instructions:u,cycles:u,task-clock,cpu-clock,faults")
                    .arg("--log-fd")
                    .arg("1")
                    .arg("setarch")
                    .arg("x86_64")
                    .arg("-R")
                    .arg(&tool)
                    .args(&args);

                let prof_out_dir = std::env::current_dir().unwrap().join("self-profile-output");
                if wrapper == "perf-stat-self-profile" {
                    cmd.arg(&format!(
                        "-Zself-profile={}",
                        prof_out_dir.to_str().unwrap()
                    ));
                    let _ = fs::remove_dir_all(&prof_out_dir);
                    let _ = fs::create_dir_all(&prof_out_dir);
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
                    for entry in fs::read_dir(&prof_out_dir).unwrap() {
                        let entry = entry.unwrap();
                        if entry
                            .file_name()
                            .to_str()
                            .map_or(false, |s| s.starts_with(crate_name))
                        {
                            let file = entry.file_name().to_str().unwrap().to_owned();
                            let new_prefix = Some(file[..file.find('.').unwrap()].to_owned());
                            assert!(
                                prefix.is_none() || prefix == new_prefix,
                                "prefix={:?}, new_prefix={:?}",
                                prefix,
                                new_prefix
                            );
                            prefix = new_prefix;
                        }
                    }
                    let prefix = prefix.expect(&format!("found prefix {:?}", prof_out_dir));
                    let json = run_summarize("summarize", &prof_out_dir, &prefix)
                        .or_else(|_| run_summarize("summarize-0.7", &prof_out_dir, &prefix))
                        .expect("able to run summarize or summarize-0.7");
                    println!("!self-profile-dir:{}", prof_out_dir.to_str().unwrap());
                    println!("!self-profile-prefix:{}", prefix);
                    println!("!self-profile-output:{}", json);
                }
            }

            "self-profile" => {
                let mut cmd = Command::new(&tool);
                cmd.arg("-Zself-profile=Zsp").args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "time-passes" => {
                args.insert(0, "-Ztime-passes".into());
                let mut cmd = bash_command(tool, args, "> Ztp");

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "perf-record" => {
                let mut cmd = Command::new("perf");
                let has_perf = cmd.output().is_ok();
                assert!(has_perf);
                cmd.arg("record")
                    .arg("--call-graph=dwarf")
                    .arg("--output=perf")
                    .arg("--freq=299")
                    .arg("--event=cycles:u,instructions:u")
                    .arg(&tool)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "oprofile" => {
                let mut cmd = Command::new("operf");
                let has_oprofile = cmd.output().is_ok();
                assert!(has_oprofile);
                // Other possibly useful args: --callgraph, --separate-thread
                cmd.arg("operf").arg(&tool).args(&args);

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
                    .arg(&tool)
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
                    .arg(&tool)
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
                    .arg(&tool)
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
                    .arg(&tool)
                    .args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "eprintln" => {
                let mut cmd = bash_command(tool, args, "2> eprintln");

                assert!(cmd.status().expect("failed to spawn").success());
            }

            "llvm-lines" => {
                // `cargo llvm-lines` writes its output to stdout. But we can't
                // redirect it to a file here like we do for "time-passes" and
                // "eprintln". This is because `cargo llvm-lines` invokes rustc
                // as part of its processing and then it does some analysis of
                // its output and then it prints out some results. Because
                // `rustc` (which this file wraps) doesn't produce the output,
                // this file can't redirect that output.
                let mut cmd = Command::new(&tool);
                cmd.args(&args);

                assert!(cmd.status().expect("failed to spawn").success());
            }

            _ => {
                panic!("unknown wrapper: {}", wrapper);
            }
        }
    } else if let Some(_) = args.iter().position(|arg| arg == "--skip-this-rustc") {
        // do nothing
    } else {
        // Abort if non-wrapped rustc
        if env::var_os("EXPECT_ONLY_WRAPPED_RUSTC").is_some() {
            // ... but not if we're just getting the rustc version.
            if !args
                .iter()
                .any(|arg| arg == "-vV" || arg == "--print=file-names")
            {
                eprintln!("{:?} {:?}", tool, args);
                eprintln!("exiting -- non-wrapped rustc");
                std::process::exit(1);
            }
        }

        let mut cmd = Command::new(&tool);
        cmd.args(&args);
        exec(&mut cmd);
    }
}

/// Run a command via bash, in order to redirect its output to a file.
/// `redirect` should be something like "> out" or "2> out".
fn bash_command(tool: OsString, args: Vec<OsString>, redirect: &str) -> Command {
    let mut bash_cmd = String::new();
    bash_cmd.push_str(&format!("{} ", tool.to_str().unwrap()));
    for arg in args {
        // Args with double quotes (e.g. `--cfg feature="foo"`)
        // will be munged by bash if we don't protect them. So we
        // wrap every arg in single quotes.
        bash_cmd.push_str(&format!("'{}' ", arg.to_str().unwrap()));
    }
    bash_cmd.push_str(redirect);

    let mut cmd = Command::new("bash");
    cmd.args(&["-c", &bash_cmd]);
    cmd
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> ! {
    use std::os::unix::prelude::*;
    let cmd_d = format!("{:?}", cmd);
    let error = cmd.exec();
    panic!("failed to exec `{}`: {}", cmd_d, error);
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

fn run_summarize(name: &str, prof_out_dir: &Path, prefix: &str) -> std::io::Result<String> {
    let mut cmd = Command::new(name);
    cmd.current_dir(&prof_out_dir);
    cmd.arg("summarize").arg("--json");
    cmd.arg(&prefix);
    let status = cmd.status()?;
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to run successfully",
        ));
    }
    fs::read_to_string(prof_out_dir.join(&format!("{}.json", prefix)))
}

#[cfg(windows)]
fn raise_priority() {}

#[cfg(windows)]
fn print_memory() {}
