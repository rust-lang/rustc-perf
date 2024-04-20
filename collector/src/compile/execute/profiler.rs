use crate::compile::execute::{PerfTool, ProcessOutputData, Processor, Retry};
use crate::utils;
use crate::utils::cachegrind::cachegrind_annotate;
use anyhow::Context;
use std::collections::HashMap;
use std::future::Future;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::pin::Pin;
use std::process::Command;
use std::{fs, io, process};

// Tools usable with the profiling subcommands, and named on the command line.
#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum Profiler {
    SelfProfile,
    PerfRecord,
    Oprofile,
    Samply,
    Cachegrind,
    Callgrind,
    Dhat,
    DhatCopy,
    Massif,
    Bytehound,
    Eprintln,
    LlvmLines,
    MonoItems,
    DepGraph,
    LlvmIr,
}

impl Profiler {
    /// Returns true if this profiler can be executed
    /// in parallel without distorting the profile results.
    pub fn supports_parallel_execution(&self) -> bool {
        matches!(
            self,
            Profiler::Cachegrind
                | Profiler::Callgrind
                | Profiler::Dhat
                | Profiler::DhatCopy
                | Profiler::Massif
                | Profiler::Bytehound
                | Profiler::Eprintln
                | Profiler::LlvmLines
                | Profiler::LlvmIr
                | Profiler::MonoItems
                | Profiler::DepGraph
        )
    }
}

pub struct ProfileProcessor<'a> {
    profiler: Profiler,
    output_dir: &'a Path,
    id: &'a str,
}

impl<'a> ProfileProcessor<'a> {
    pub fn new(profiler: Profiler, output_dir: &'a Path, id: &'a str) -> Self {
        ProfileProcessor {
            profiler,
            output_dir,
            id,
        }
    }
}

impl<'a> Processor for ProfileProcessor<'a> {
    fn perf_tool(&self) -> PerfTool {
        PerfTool::ProfileTool(self.profiler)
    }

    fn process_output<'b>(
        &'b mut self,
        data: &'b ProcessOutputData<'_>,
        output: process::Output,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Retry>> + 'b>> {
        Box::pin(async move {
            fs::create_dir_all(self.output_dir)?;

            // Produce a name of the form $PREFIX-$ID-$BENCHMARK-$PROFILE-$SCENARIO.
            let out_file = |prefix: &str| -> String {
                format!(
                    "{}-{}-{}-{:?}-{}",
                    prefix, self.id, data.name, data.profile, data.scenario_str
                )
            };

            // Combine a dir and a file.
            let filepath = |dir: &Path, file: &str| {
                let mut path = dir.to_path_buf();
                path.push(file);
                path
            };

            match self.profiler {
                // -Zself-profile produces (via rustc-fake) a data directory called
                // `Zsp` containing three files with names of the form
                // `$BENCHMARK-$PID.{events,string_data,string_index}`. We copy it
                // from the temp dir to the output dir, renaming the files within
                // as `Zsp.{events,string_data,string_index}` in the process, then
                // post-process them with `summarize`, `flamegraph`, and `crox` to
                // produce several data files in the output dir.
                Profiler::SelfProfile => {
                    let tmp_zsp_dir = filepath(data.cwd, "Zsp");
                    let zsp_dir = filepath(self.output_dir, &out_file("Zsp"));
                    let zsp_files_prefix = filepath(&zsp_dir, "Zsp");
                    let summarize_file = filepath(self.output_dir, &out_file("summarize"));
                    let flamegraph_file = filepath(self.output_dir, &out_file("flamegraph"));
                    let crox_file = filepath(self.output_dir, &out_file("crox"));

                    // Move the directory.
                    if zsp_dir.exists() {
                        fs::remove_dir_all(&zsp_dir)?;
                    }

                    utils::fs::rename(tmp_zsp_dir, &zsp_dir)?;

                    // Rename the data files. There should be exactly three.
                    let mut num_files = 0;
                    for entry in fs::read_dir(&zsp_dir).unwrap() {
                        num_files += 1;
                        let filename = entry.unwrap().file_name();
                        let filename_str = filename.to_str().unwrap();
                        let path = filepath(&zsp_dir, filename_str);
                        if filename_str.ends_with(".events") {
                            utils::fs::rename(path, filepath(&zsp_dir, "Zsp.events"))?;
                        } else if filename_str.ends_with(".string_data") {
                            utils::fs::rename(path, filepath(&zsp_dir, "Zsp.string_data"))?;
                        } else if filename_str.ends_with(".string_index") {
                            utils::fs::rename(path, filepath(&zsp_dir, "Zsp.string_index"))?;
                        } else if filename_str.ends_with(".mm_profdata") {
                            utils::fs::rename(path, filepath(&zsp_dir, "Zsp.mm_profdata"))?;
                        } else {
                            panic!("unexpected file {:?}", path);
                        }
                    }
                    assert!(num_files == 3 || num_files == 1);

                    // Run `summarize`.
                    let mut summarize_cmd = Command::new("summarize");
                    summarize_cmd.arg("summarize").arg(&zsp_files_prefix);
                    fs::write(
                        summarize_file,
                        summarize_cmd.output().context("summarize")?.stdout,
                    )?;

                    // Run `flamegraph`.
                    let mut flamegraph_cmd = Command::new("flamegraph");
                    flamegraph_cmd.arg(&zsp_files_prefix);
                    flamegraph_cmd.status().context("flamegraph")?;
                    utils::fs::rename("rustc.svg", flamegraph_file)?;

                    // Run `crox`.
                    let mut crox_cmd = Command::new("crox");
                    crox_cmd.arg(&zsp_files_prefix);
                    crox_cmd.status().context("crox")?;
                    utils::fs::rename("chrome_profiler.json", crox_file)?;
                }

                // perf-record produces (via rustc-fake) a data file called `perf`.
                // We copy it from the temp dir to the output dir, giving it a new
                // name in the process.
                Profiler::PerfRecord => {
                    let tmp_perf_file = filepath(data.cwd, "perf");
                    let perf_file = filepath(self.output_dir, &out_file("perf"));

                    fs::copy(tmp_perf_file, perf_file)?;
                }

                // OProfile produces (via rustc-fake) a data directory called
                // `oprofile_data`. We copy it from the temp dir to the output dir,
                // giving it a new name in the process, and then post-process it
                // twice to produce another two data files in the output dir.
                Profiler::Oprofile => {
                    let tmp_opout_dir = filepath(data.cwd, "oprofile_data");
                    let opout_dir = filepath(self.output_dir, &out_file("opout"));
                    let oprep_file = filepath(self.output_dir, &out_file("oprep"));
                    let opann_file = filepath(self.output_dir, &out_file("opann"));

                    // Move the directory.
                    if opout_dir.exists() {
                        fs::remove_dir_all(&opout_dir)?;
                    }
                    utils::fs::rename(tmp_opout_dir, &opout_dir)?;

                    let mut session_dir_arg = "--session-dir=".to_string();
                    session_dir_arg.push_str(opout_dir.to_str().unwrap());

                    let mut op_report_cmd = Command::new("opreport");
                    // Other possibly useful args: --callgraph (requires
                    // --callgraph for operf), --details
                    op_report_cmd
                        .arg("--symbols")
                        .arg("--debug-info")
                        .arg("--threshold")
                        .arg("0.5")
                        .arg(&session_dir_arg);
                    fs::write(oprep_file, op_report_cmd.output()?.stdout)?;

                    let mut op_annotate_cmd = Command::new("opannotate");
                    // Other possibly useful args: --assembly
                    op_annotate_cmd
                        .arg("--source")
                        .arg("--threshold")
                        .arg("0.5")
                        .arg(&session_dir_arg);
                    fs::write(opann_file, op_annotate_cmd.output()?.stdout)?;
                }

                // Samply produces (via rustc-fake) a data file called
                // `profile.json`. We copy it from the temp dir to the output dir,
                // giving it a new name in the process.
                Profiler::Samply => {
                    let tmp_samply_file = filepath(data.cwd, "profile.json");
                    let samply_file = filepath(self.output_dir, &out_file("samply"));

                    fs::copy(tmp_samply_file, samply_file)?;
                }

                // Cachegrind produces (via rustc-fake) a data file called `cgout`.
                // We copy it from the temp dir to the output dir, giving it a new
                // name in the process, and then post-process it to produce another
                // data file in the output dir.
                Profiler::Cachegrind => {
                    let tmp_cgout_file = filepath(data.cwd, "cgout");
                    let cgout_file = filepath(self.output_dir, &out_file("cgout"));
                    let cgann_file = filepath(self.output_dir, &out_file("cgann"));

                    cachegrind_annotate(&tmp_cgout_file, &cgout_file, &cgann_file)?;
                }

                // Callgrind produces (via rustc-fake) a data file called `clgout`.
                // We copy it from the temp dir to the output dir, giving it a new
                // name in the process, and then post-process it to produce another
                // data file in the output dir.
                Profiler::Callgrind => {
                    let tmp_clgout_file = filepath(data.cwd, "clgout");
                    let clgout_file = filepath(self.output_dir, &out_file("clgout"));
                    let clgann_file = filepath(self.output_dir, &out_file("clgann"));

                    fs::copy(tmp_clgout_file, &clgout_file)?;

                    let mut clg_annotate_cmd = Command::new("callgrind_annotate");
                    clg_annotate_cmd
                        .arg("--auto=yes")
                        .arg("--show-percs=yes")
                        .arg(&clgout_file);
                    fs::write(clgann_file, clg_annotate_cmd.output()?.stdout)?;
                }

                // DHAT produces (via rustc-fake) a data file called `dhout`. We
                // copy it from the temp dir to the output dir, giving it a new
                // name in the process.
                Profiler::Dhat => {
                    let tmp_dhout_file = filepath(data.cwd, "dhout");
                    let dhout_file = filepath(self.output_dir, &out_file("dhout"));

                    fs::copy(tmp_dhout_file, dhout_file)?;
                }

                // DHAT (in copy mode) produces (via rustc-fake) a data file called
                // `dhcopy`. We copy it from the temp dir to the output dir, giving
                // it a new name in the process.
                Profiler::DhatCopy => {
                    let tmp_dhcopy_file = filepath(data.cwd, "dhcopy");
                    let dhcopy_file = filepath(self.output_dir, &out_file("dhcopy"));

                    fs::copy(tmp_dhcopy_file, dhcopy_file)?;
                }

                // Massif produces (via rustc-fake) a data file called `msout`. We
                // copy it from the temp dir to the output dir, giving it a new
                // name in the process.
                Profiler::Massif => {
                    let tmp_msout_file = filepath(data.cwd, "msout");
                    let msout_file = filepath(self.output_dir, &out_file("msout"));

                    fs::copy(tmp_msout_file, msout_file)?;
                }

                // Bytehound produces (via rustc-fake) a data file called
                // `bytehound.dat`. We copy it from the temp dir to the output dir, giving
                // it a new name in the process.
                Profiler::Bytehound => {
                    let tmp_bytehound_file = filepath(data.cwd, "bytehound.dat");
                    let target_file = filepath(self.output_dir, &out_file("bhout"));
                    fs::copy(tmp_bytehound_file, target_file)?;
                }

                // `eprintln!` statements are redirected (via rustc-fake) to a file
                // called `eprintln`. We copy it from the temp dir to the output
                // dir, giving it a new name in the process.
                Profiler::Eprintln => {
                    let tmp_eprintln_file = filepath(data.cwd, "eprintln");
                    let eprintln_file = filepath(self.output_dir, &out_file("eprintln"));

                    #[allow(dead_code)]
                    #[derive(serde::Deserialize)]
                    struct RustcMessage<'a> {
                        #[serde(rename = "$message_type")]
                        message_type: &'a str,
                    }

                    let mut final_file = io::BufWriter::new(std::fs::File::create(eprintln_file)?);
                    for line in io::BufReader::new(std::fs::File::open(tmp_eprintln_file)?).lines()
                    {
                        let line = line?;

                        // rustc under Cargo currently ~always emits artifact
                        // messages -- which we don't want in final
                        // eprintln output. These messages contain a $message_type tag since
                        // https://github.com/rust-lang/rust/pull/115691.
                        if serde_json::from_str::<RustcMessage>(&line).is_ok() {
                            continue;
                        }

                        writeln!(&mut final_file, "{}", line)?;
                    }
                }

                // mono item results are redirected (via rustc-fake) to a file
                // called `mono-items`. We copy it from the temp dir to the output
                // dir, giving it a new name in the process.
                Profiler::MonoItems => {
                    let tmp_file = filepath(data.cwd, "mono-items");
                    let out_dir = self.output_dir.join(out_file("mono-items"));
                    let _ = fs::create_dir_all(&out_dir);
                    let result_file = filepath(&out_dir, "raw");

                    fs::copy(&tmp_file, result_file)?;

                    let mut by_cgu: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
                    let mono_items = std::fs::read_to_string(&tmp_file)?;
                    for line in mono_items.lines() {
                        let line = if let Some(line) = line.strip_prefix("MONO_ITEM ") {
                            line
                        } else {
                            continue;
                        };

                        let (name, cgus) = if let Some(parts) = line.split_once(" @@ ") {
                            parts
                        } else {
                            continue;
                        };

                        for cgu in cgus.split(' ') {
                            let cgu_name_end = cgu.rfind('[').expect(cgu);
                            let cgu_name = &cgu[..cgu_name_end];
                            let linkage = &cgu[cgu_name_end + 1..cgu.len() - 1];
                            by_cgu.entry(cgu_name).or_default().push((name, linkage));
                        }
                    }

                    for (cgu, items) in &by_cgu {
                        let cgu_file = filepath(&out_dir, cgu);
                        let mut file = io::BufWriter::new(
                            fs::File::create(&cgu_file)
                                .with_context(|| format!("{:?}", cgu_file))?,
                        );
                        for (name, linkage) in items {
                            writeln!(&mut file, "{} {}", name, linkage)?;
                        }
                    }
                }

                Profiler::DepGraph => {
                    let tmp_file = filepath(data.cwd, "dep_graph.txt");
                    let output =
                        filepath(self.output_dir, &out_file("dep-graph")).with_extension("txt");

                    fs::copy(tmp_file, output)?;

                    let tmp_file = filepath(data.cwd, "dep_graph.dot");
                    let output =
                        filepath(self.output_dir, &out_file("dep-graph")).with_extension("dot");

                    // May not exist if not incremental, but then that's OK.
                    fs::copy(tmp_file, output)?;
                }

                Profiler::LlvmIr => {
                    let tmp_file = filepath(data.cwd, "llvm-ir");
                    let output = filepath(self.output_dir, &out_file("llir"));
                    fs::copy(tmp_file, output)?;
                }

                // `cargo llvm-lines` writes its output to stdout. We copy that
                // output into a file in the output dir.
                Profiler::LlvmLines => {
                    let ll_file = filepath(self.output_dir, &out_file("ll"));

                    fs::write(ll_file, output.stdout)?;
                }
            }
            Ok(Retry::No)
        })
    }
}
