//! Execute benchmarks.

use crate::{BuildKind, Compiler, RunKind};
use anyhow::{anyhow, bail, Context};
use collector::command_output;
use database::{PatchName, QueryLabel};
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::hash;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::str;
use std::time::Duration;
use tempfile::TempDir;
use tokio::runtime::Runtime;

fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> anyhow::Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    if fs::rename(from, to).is_err() {
        // This is necessary if from and to are on different
        // mount points (e.g., if /tmp is in tmpfs instead of on
        // the same disk). We don't want to implement a full recursive solution
        // to copying directories, so just shell out to `mv`.
        let ctx = format!("mv {:?} {:?}", from, to);
        let status = Command::new("mv")
            .arg(from)
            .arg(to)
            .status()
            .with_context(|| ctx.clone())?;
        if !status.success() {
            anyhow::bail!("mv {:?} {:?}: {:?}", from, to, status);
        }
    }

    Ok(())
}

fn touch(root: &Path, path: &Path) -> anyhow::Result<()> {
    let mut cmd = Command::new("touch");
    cmd.current_dir(root).arg(path);
    command_output(&mut cmd).with_context(|| format!("touching {:?} in {:?}", path, root))?;
    Ok(())
}

fn touch_all(path: &Path) -> anyhow::Result<()> {
    let mut cmd = Command::new("bash");
    cmd.current_dir(path)
        .args(&["-c", "find . -name '*.rs' | xargs touch"]);
    command_output(&mut cmd).with_context(|| format!("touching all .rs in {:?}", path))?;
    // We also delete the cmake caches to avoid errors when moving directories around.
    // This might be a bit slower but at least things build
    let mut cmd = Command::new("bash");
    cmd.current_dir(path)
        .args(&["-c", "find . -name 'CMakeCache.txt' -delete"]);
    command_output(&mut cmd).with_context(|| format!("deleting cmake caches in {:?}", path))?;
    Ok(())
}

fn default_runs() -> usize {
    3
}

/// This is the internal representation of an individual benchmark's
/// perf-config.json file.
#[derive(Debug, Clone, serde::Deserialize)]
struct BenchmarkConfig {
    cargo_opts: Option<String>,
    cargo_rustc_opts: Option<String>,
    cargo_toml: Option<String>,
    #[serde(default)]
    disabled: bool,
    #[serde(default = "default_runs")]
    runs: usize,
    #[serde(default)]
    supports_stable: bool,

    /// The file that should be touched to ensure cargo re-checks the leaf crate
    /// we're interested in. Likely, something similar to `src/lib.rs`. The
    /// default if this is not present is to touch all .rs files in the
    /// directory that `Cargo.toml` is in.
    #[serde(default)]
    touch_file: Option<String>,
}

impl Default for BenchmarkConfig {
    fn default() -> BenchmarkConfig {
        BenchmarkConfig {
            cargo_opts: None,
            cargo_rustc_opts: None,
            cargo_toml: None,
            disabled: false,
            runs: default_runs(),
            supports_stable: false,
            touch_file: None,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct BenchmarkName(pub String);

impl fmt::Display for BenchmarkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Benchmark {
    pub name: BenchmarkName,
    pub path: PathBuf,
    patches: Vec<Patch>,
    config: BenchmarkConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Profiler {
    PerfStat,
    PerfStatSelfProfile,
    SelfProfile,
    TimePasses,
    PerfRecord,
    OProfile,
    Cachegrind,
    Callgrind,
    DHAT,
    Massif,
    Eprintln,
    LlvmLines,
}

impl Profiler {
    pub fn from_name(name: &str) -> anyhow::Result<Profiler> {
        match name {
            // Even though `PerfStat` is a valid `Profiler` value, "perf-stat"
            // is rejected because it can't be used with the `profiler`
            // subcommand. (It's used with `bench_local` instead.)
            "perf-stat" => Err(anyhow!("'perf-stat' cannot be used as the profiler")),
            "self-profile" => Ok(Profiler::SelfProfile),
            "time-passes" => Ok(Profiler::TimePasses),
            "perf-record" => Ok(Profiler::PerfRecord),
            "oprofile" => Ok(Profiler::OProfile),
            "cachegrind" => Ok(Profiler::Cachegrind),
            "callgrind" => Ok(Profiler::Callgrind),
            "dhat" => Ok(Profiler::DHAT),
            "massif" => Ok(Profiler::Massif),
            "eprintln" => Ok(Profiler::Eprintln),
            "llvm-lines" => Ok(Profiler::LlvmLines),
            _ => Err(anyhow!("'{}' is not a known profiler", name)),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Profiler::PerfStat => "perf-stat",
            Profiler::PerfStatSelfProfile => "perf-stat-self-profile",
            Profiler::SelfProfile => "self-profile",
            Profiler::TimePasses => "time-passes",
            Profiler::PerfRecord => "perf-record",
            Profiler::OProfile => "oprofile",
            Profiler::Cachegrind => "cachegrind",
            Profiler::Callgrind => "callgrind",
            Profiler::DHAT => "dhat",
            Profiler::Massif => "massif",
            Profiler::Eprintln => "eprintln",
            Profiler::LlvmLines => "llvm-lines",
        }
    }

    // What cargo subcommand do we need to run for this profiler? If not
    // `rustc`, must be a subcommand that itself invokes `rustc`.
    fn subcommand(&self, build_kind: BuildKind) -> Option<&'static str> {
        match self {
            Profiler::PerfStat
            | Profiler::PerfStatSelfProfile
            | Profiler::SelfProfile
            | Profiler::TimePasses
            | Profiler::PerfRecord
            | Profiler::OProfile
            | Profiler::Cachegrind
            | Profiler::Callgrind
            | Profiler::DHAT
            | Profiler::Massif
            | Profiler::Eprintln => {
                if build_kind == BuildKind::Doc {
                    Some("rustdoc")
                } else {
                    Some("rustc")
                }
            }
            Profiler::LlvmLines => match build_kind {
                BuildKind::Debug | BuildKind::Opt => Some("llvm-lines"),
                BuildKind::Check | BuildKind::Doc => None,
            },
        }
    }

    fn is_run_kind_allowed(&self, run_kind: RunKind) -> bool {
        match self {
            Profiler::PerfStat
            | Profiler::PerfStatSelfProfile
            | Profiler::SelfProfile
            | Profiler::TimePasses
            | Profiler::PerfRecord
            | Profiler::OProfile
            | Profiler::Cachegrind
            | Profiler::Callgrind
            | Profiler::DHAT
            | Profiler::Massif
            | Profiler::Eprintln => true,
            Profiler::LlvmLines => run_kind == RunKind::Full,
        }
    }
}

struct CargoProcess<'a> {
    compiler: Compiler<'a>,
    cwd: &'a Path,
    build_kind: BuildKind,
    incremental: bool,
    processor_etc: Option<(&'a mut dyn Processor, RunKind, &'a str, Option<&'a Patch>)>,
    processor_name: BenchmarkName,
    manifest_path: String,
    cargo_args: Vec<String>,
    rustc_args: Vec<String>,
    touch_file: Option<String>,
    jobserver: Option<jobserver::Client>,
}

impl<'a> CargoProcess<'a> {
    fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    fn processor(
        mut self,
        processor: &'a mut dyn Processor,
        run_kind: RunKind,
        run_kind_str: &'a str,
        patch: Option<&'a Patch>,
    ) -> Self {
        self.processor_etc = Some((processor, run_kind, run_kind_str, patch));
        self
    }

    fn base_command(&self, cwd: &Path, subcommand: &str) -> Command {
        let mut cmd = Command::new(Path::new(self.compiler.cargo));
        cmd
            // Not all cargo invocations (e.g. `cargo clean`) need all of these
            // env vars set, but it doesn't hurt to have them.
            .env("RUSTC", &*FAKE_RUSTC)
            .env("RUSTC_REAL", &self.compiler.rustc)
            // We separately pass -Cincremental to the leaf crate --
            // CARGO_INCREMENTAL is cached separately for both the leaf crate
            // and any in-tree dependencies, and we don't want that; it wastes
            // time.
            .env("CARGO_INCREMENTAL", "0")
            .current_dir(cwd)
            .arg(subcommand)
            .arg("--manifest-path")
            .arg(&self.manifest_path);

        if let Some(r) = &self.compiler.rustdoc {
            cmd.env("RUSTDOC", &*FAKE_RUSTDOC).env("RUSTDOC_REAL", r);
        }
        cmd
    }

    fn get_pkgid(&self, cwd: &Path) -> anyhow::Result<String> {
        let mut pkgid_cmd = self.base_command(cwd, "pkgid");
        let out = command_output(&mut pkgid_cmd)
            .with_context(|| format!("failed to obtain pkgid in '{:?}'", cwd))?
            .stdout;
        let package_id = str::from_utf8(&out).unwrap();
        Ok(package_id.trim().to_string())
    }

    fn jobserver(mut self, server: jobserver::Client) -> Self {
        self.jobserver = Some(server);
        self
    }

    // FIXME: the needs_final and processor_etc interactions aren't ideal; we
    // would like to "auto know" when we need final but currently we don't
    // really.
    fn run_rustc(&mut self, needs_final: bool) -> anyhow::Result<()> {
        log::info!(
            "run_rustc with incremental={}, build_kind={:?}, run_kind={:?}, patch={:?}",
            self.incremental,
            self.build_kind,
            self.processor_etc.as_ref().map(|v| v.1),
            self.processor_etc.as_ref().and_then(|v| v.3)
        );

        loop {
            // Get the subcommand. If it's not `rustc` it must should be a
            // subcommand that itself invokes `rustc` (so that the `FAKE_RUSTC`
            // machinery works).
            let subcommand = if let Some((ref mut processor, run_kind, ..)) = self.processor_etc {
                let profiler = processor.profiler(self.build_kind);
                if !profiler.is_run_kind_allowed(run_kind) {
                    return Err(anyhow::anyhow!(
                        "this profiler doesn't support {:?} runs",
                        run_kind
                    ));
                }

                match profiler.subcommand(self.build_kind) {
                    None => {
                        return Err(anyhow::anyhow!(
                            "this profiler doesn't support {:?} builds",
                            self.build_kind
                        ))
                    }
                    Some(sub) => sub,
                }
            } else {
                match self.build_kind {
                    BuildKind::Doc => "rustdoc",
                    _ => "rustc",
                }
            };

            let mut cmd = self.base_command(self.cwd, subcommand);
            cmd.arg("-p").arg(self.get_pkgid(self.cwd)?);
            match self.build_kind {
                BuildKind::Check => {
                    cmd.arg("--profile").arg("check");
                }
                BuildKind::Debug => {}
                BuildKind::Doc => {}
                BuildKind::Opt => {
                    cmd.arg("--release");
                }
            }
            cmd.args(&self.cargo_args);
            if env::var_os("CARGO_RECORD_TIMING").is_some() {
                cmd.arg("-Zunstable-options");
                cmd.arg("-Ztimings");
            }
            cmd.arg("--");
            // --wrap-rustc-with is not a valid rustc flag. But rustc-fake
            // recognizes it, strips it (and its argument) out, and uses it as an
            // indicator that the rustc invocation should be profiled. This works
            // out nicely because `cargo rustc` only passes arguments after '--'
            // onto rustc for the final crate, which is exactly the crate for which
            // we want to wrap rustc.
            if needs_final {
                let processor = self
                    .processor_etc
                    .as_mut()
                    .map(|v| &mut v.0)
                    .expect("needs_final needs a processor");
                let profiler = processor.profiler(self.build_kind).name();
                // If we're using a processor, we expect that only the crate
                // we're interested in benchmarking will be built, not any
                // dependencies.
                cmd.env("EXPECT_ONLY_WRAPPED_RUSTC", "1");
                cmd.arg("--wrap-rustc-with");
                cmd.arg(profiler);
                cmd.args(&self.rustc_args);

                // If we're not going to be in a processor, then there's no
                // point ensuring that we recompile anything -- that just wastes
                // time.

                // Touch all the files under the Cargo.toml of the manifest we're
                // benchmarking, so as to not refresh dependencies, which may be
                // in-tree (e.g., in the case of the servo crates there are a lot of
                // other components).
                if let Some(file) = &self.touch_file {
                    touch(&self.cwd, Path::new(&file))?;
                } else {
                    touch_all(
                        &self.cwd.join(
                            Path::new(&self.manifest_path)
                                .parent()
                                .expect("manifest has parent"),
                        ),
                    )?;
                }
            } else {
                // If we're not going to record the final rustc, then there's
                // absolutely no point in waiting for it to build. This will
                // have the final rustc just immediately exit(0) without
                // actually running it.
                cmd.arg("--skip-this-rustc");
            }

            if self.incremental {
                cmd.arg("-C");
                let mut incr_arg = std::ffi::OsString::from("incremental=");
                incr_arg.push(self.cwd.join("incremental-state"));
                cmd.arg(incr_arg);
            }

            if let Some(client) = &self.jobserver {
                client.configure(&mut cmd);
            }

            log::debug!("{:?}", cmd);

            let output = command_output(&mut cmd)?;
            if let Some((ref mut processor, run_kind, run_kind_str, patch)) = self.processor_etc {
                let data = ProcessOutputData {
                    name: self.processor_name.clone(),
                    cwd: self.cwd,
                    build_kind: self.build_kind,
                    run_kind,
                    run_kind_str,
                    patch,
                };
                match processor.process_output(&data, output) {
                    Ok(Retry::No) => return Ok(()),
                    Ok(Retry::Yes) => {}
                    Err(e) => return Err(e),
                }
            } else {
                return Ok(());
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref FAKE_RUSTC: PathBuf = {
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        fake_rustc
    };
    static ref FAKE_RUSTDOC: PathBuf = {
        let mut fake_rustdoc = env::current_exe().unwrap();
        fake_rustdoc.pop();
        fake_rustdoc.push("rustdoc-fake");
        // link from rustc-fake to rustdoc-fake
        if !fake_rustdoc.exists() {
            #[cfg(unix)]
            use std::os::unix::fs::symlink;
            #[cfg(windows)]
            use std::os::windows::fs::symlink_file as symlink;

            symlink(&*FAKE_RUSTC, &fake_rustdoc).expect("failed to make symbolic link");
        }
        fake_rustdoc
    };
}

/// Used to indicate if we need to retry a run.
pub enum Retry {
    No,
    Yes,
}

pub struct ProcessOutputData<'a> {
    name: BenchmarkName,
    cwd: &'a Path,
    build_kind: BuildKind,
    run_kind: RunKind,
    run_kind_str: &'a str,
    patch: Option<&'a Patch>,
}

/// Trait used by `Benchmark::measure()` to provide different kinds of
/// processing.
pub trait Processor {
    /// The `Profiler` being used.
    fn profiler(&self, _: BuildKind) -> Profiler;

    /// Process the output produced by the particular `Profiler` being used.
    fn process_output(
        &mut self,
        data: &ProcessOutputData<'_>,
        output: process::Output,
    ) -> anyhow::Result<Retry>;

    /// Provided to permit switching on more expensive profiling if it's needed
    /// for the "first" run for any given benchmark (we reuse the processor),
    /// e.g. disabling -Zself-profile.
    fn start_first_collection(&mut self) {}

    /// Provided to permit switching off more expensive profiling if it's needed
    /// for the "first" run, e.g. disabling -Zself-profile.
    ///
    /// Return "true" if planning on doing something different for second
    /// iteration.
    fn finished_first_collection(&mut self, _: BuildKind) -> bool {
        false
    }
}

pub struct MeasureProcessor<'a> {
    rt: &'a mut Runtime,
    krate: &'a BenchmarkName,
    conn: &'a mut dyn database::Connection,
    cid: database::ArtifactIdNumber,
    upload: Option<Upload>,
    is_first_collection: bool,
    self_profile: bool,
    tries: u8,
}

impl<'a> MeasureProcessor<'a> {
    pub fn new(
        rt: &'a mut Runtime,
        conn: &'a mut dyn database::Connection,
        krate: &'a BenchmarkName,
        cid: database::ArtifactIdNumber,
        self_profile: bool,
    ) -> Self {
        // Check we have `perf` available.
        let has_perf = Command::new("perf").output().is_ok();
        assert!(has_perf);

        MeasureProcessor {
            rt,
            upload: None,
            conn,
            krate,
            cid,
            is_first_collection: true,
            // Command::new("summarize").status().is_ok()
            self_profile,
            tries: 0,
        }
    }

    fn insert_stats(
        &mut self,
        cache: database::Cache,
        build_kind: BuildKind,
        stats: (Stats, Option<SelfProfile>, Option<SelfProfileFiles>),
    ) {
        let version = String::from_utf8(
            Command::new("git")
                .arg("rev-parse")
                .arg("HEAD")
                .output()
                .context("git rev-parse HEAD")
                .unwrap()
                .stdout,
        )
        .context("utf8")
        .unwrap();

        let collection = self.rt.block_on(self.conn.collection_id(&version));
        let profile = match build_kind {
            BuildKind::Check => database::Profile::Check,
            BuildKind::Debug => database::Profile::Debug,
            BuildKind::Doc => database::Profile::Doc,
            BuildKind::Opt => database::Profile::Opt,
        };

        if let Some(files) = stats.2 {
            if env::var_os("RUSTC_PERF_UPLOAD_TO_S3").is_some() {
                // We can afford to have the uploads run concurrently with
                // rustc. Generally speaking, they take up almost no CPU time
                // (just copying data into the network). Plus, during
                // self-profile data timing noise doesn't matter as much. (We'll
                // be migrating to instructions soon, hopefully, where the
                // upload will cause even less noise). We may also opt at some
                // point to defer these uploads entirely to the *end* or
                // something like that. For now though this works quite well.
                if let Some(u) = self.upload.take() {
                    u.wait();
                }
                let prefix = PathBuf::from("self-profile")
                    .join(self.cid.0.to_string())
                    .join(self.krate.0.as_str())
                    .join(profile.to_string())
                    .join(cache.to_id());
                self.upload = Some(Upload::new(prefix, collection, files));
                self.rt.block_on(self.conn.record_raw_self_profile(
                    collection,
                    self.cid,
                    self.krate.0.as_str(),
                    profile,
                    cache,
                ));
            }
        }

        let mut buf = FuturesUnordered::new();
        for (stat, value) in stats.0.iter() {
            buf.push(self.conn.record_statistic(
                collection,
                self.cid,
                self.krate.0.as_str(),
                profile,
                cache,
                stat,
                value,
            ));
        }

        if let Some(sp) = &stats.1 {
            let conn = &*self.conn;
            let cid = self.cid;
            let krate = self.krate.0.as_str();
            for qd in &sp.query_data {
                buf.push(conn.record_self_profile_query(
                    collection,
                    cid,
                    krate,
                    profile,
                    cache,
                    qd.label.as_str(),
                    database::QueryDatum {
                        self_time: qd.self_time,
                        blocked_time: qd.blocked_time,
                        incremental_load_time: qd.incremental_load_time,
                        number_of_cache_hits: qd.number_of_cache_hits,
                        invocation_count: qd.invocation_count,
                    },
                ));
            }
        }

        self.rt
            .block_on(async move { while let Some(()) = buf.next().await {} });
    }
}

struct Upload(std::process::Child, tempfile::NamedTempFile);

impl Upload {
    fn new(prefix: PathBuf, collection: database::CollectionId, files: SelfProfileFiles) -> Upload {
        // Files are placed at
        //  * self-profile/<artifact id>/<krate>/<profile>/<cache>
        //    /self-profile-<collection-id>.{extension}
        let tarball = snap::write::FrameEncoder::new(Vec::new());
        let mut builder = tar::Builder::new(tarball);
        builder.mode(tar::HeaderMode::Deterministic);

        let append_file =
            |builder: &mut tar::Builder<_>, file: &Path, name: &str| -> anyhow::Result<()> {
                builder.append_path_with_name(file, name)?;
                Ok(())
            };
        append_file(
            &mut builder,
            &files.string_index,
            "self-profile.string_index",
        )
        .expect("append string index");
        append_file(&mut builder, &files.string_data, "self-profile.string_data")
            .expect("append string data");
        append_file(&mut builder, &files.events, "self-profile.events").expect("append events");

        let upload = tempfile::NamedTempFile::new()
            .context("create temporary file")
            .unwrap();
        builder.finish().expect("complete tarball");
        std::fs::write(
            upload.path(),
            builder
                .into_inner()
                .expect("get")
                .into_inner()
                .expect("snap success"),
        )
        .expect("wrote tarball");

        let child = Command::new("aws")
            .arg("s3")
            .arg("cp")
            .arg("--only-show-errors")
            .arg(upload.path())
            .arg(&format!(
                "s3://rustc-perf/{}",
                &prefix
                    .join(format!("self-profile-{}.tar.sz", collection))
                    .to_str()
                    .unwrap()
            ))
            .spawn()
            .expect("spawn aws");

        Upload(child, upload)
    }

    fn wait(mut self) {
        let start = std::time::Instant::now();
        let status = self.0.wait().expect("waiting for child");
        if !status.success() {
            panic!("S3 upload failed: {:?}", status);
        }

        log::trace!("uploaded to S3, additional wait: {:?}", start.elapsed());
    }
}

impl<'a> Processor for MeasureProcessor<'a> {
    fn profiler(&self, build: BuildKind) -> Profiler {
        if self.is_first_collection && self.self_profile && build != BuildKind::Doc {
            Profiler::PerfStatSelfProfile
        } else {
            Profiler::PerfStat
        }
    }

    fn start_first_collection(&mut self) {
        self.is_first_collection = true;
    }

    fn finished_first_collection(&mut self, build: BuildKind) -> bool {
        let original = self.profiler(build);
        self.is_first_collection = false;
        // We need to run again if we're going to use a different profiler
        self.profiler(build) != original
    }

    fn process_output(
        &mut self,
        data: &ProcessOutputData<'_>,
        output: process::Output,
    ) -> anyhow::Result<Retry> {
        match process_perf_stat_output(output) {
            Ok(res) => {
                match data.run_kind {
                    RunKind::Full => {
                        self.insert_stats(database::Cache::Empty, data.build_kind, res);
                    }
                    RunKind::IncrFull => {
                        self.insert_stats(database::Cache::IncrementalEmpty, data.build_kind, res);
                    }
                    RunKind::IncrUnchanged => {
                        self.insert_stats(database::Cache::IncrementalFresh, data.build_kind, res);
                    }
                    RunKind::IncrPatched => {
                        let patch = data.patch.unwrap();
                        self.insert_stats(
                            database::Cache::IncrementalPatch(patch.name),
                            data.build_kind,
                            res,
                        );
                    }
                }
                Ok(Retry::No)
            }
            Err(DeserializeStatError::NoOutput(output)) => {
                if self.tries < 5 {
                    log::warn!(
                        "failed to deserialize stats, retrying (try {}); output: {:?}",
                        self.tries,
                        output
                    );
                    self.tries += 1;
                    Ok(Retry::Yes)
                } else {
                    panic!("failed to collect statistics after 5 tries");
                }
            }
            Err(e @ DeserializeStatError::ParseError { .. }) => {
                panic!("process_perf_stat_output failed: {:?}", e);
            }
        }
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
    fn profiler(&self, _: BuildKind) -> Profiler {
        self.profiler
    }

    fn process_output(
        &mut self,
        data: &ProcessOutputData<'_>,
        output: process::Output,
    ) -> anyhow::Result<Retry> {
        fs::create_dir_all(self.output_dir)?;

        // Produce a name of the form $PREFIX-$ID-$BENCHMARK-$BUILDKIND-$RUNKIND.
        let out_file = |prefix: &str| -> String {
            format!(
                "{}-{}-{}-{:?}-{}",
                prefix, self.id, data.name, data.build_kind, data.run_kind_str
            )
        };

        // Combine a dir and a file.
        let filepath = |dir: &Path, file: &str| {
            let mut path = dir.to_path_buf();
            path.push(file);
            path
        };

        match self.profiler {
            Profiler::PerfStat | Profiler::PerfStatSelfProfile => {
                panic!("unexpected profiler");
            }

            // -Zself-profile produces (via rustc-fake) a data directory called
            // `Zsp` containing three files with names of the form
            // `$BENCHMARK-$PID.{events,string_data,string_index}`. We copy it
            // from the temp dir to the output dir, renaming the files within
            // as `Zsp.{events,string_data,string_index}` in the process, then
            // post-process them with `summarize`, `flamegraph`, and `crox` to
            // produce several data files in the output dir.
            Profiler::SelfProfile => {
                let tmp_zsp_dir = filepath(data.cwd.as_ref(), "Zsp");
                let zsp_dir = filepath(self.output_dir, &out_file("Zsp"));
                let zsp_files_prefix = filepath(&zsp_dir, "Zsp");
                let summarize_file = filepath(self.output_dir, &out_file("summarize"));
                let flamegraph_file = filepath(self.output_dir, &out_file("flamegraph"));
                let crox_file = filepath(self.output_dir, &out_file("crox"));

                // Move the directory.
                if zsp_dir.exists() {
                    fs::remove_dir_all(&zsp_dir)?;
                }

                rename(&tmp_zsp_dir, &zsp_dir)?;

                // Rename the data files. There should be exactly three.
                let mut num_files = 0;
                for entry in fs::read_dir(&zsp_dir).unwrap() {
                    num_files += 1;
                    let filename = entry.unwrap().file_name();
                    let filename_str = filename.to_str().unwrap();
                    let path = filepath(&zsp_dir, filename_str);
                    if filename_str.ends_with(".events") {
                        rename(path, filepath(&zsp_dir, "Zsp.events"))?;
                    } else if filename_str.ends_with(".string_data") {
                        rename(path, filepath(&zsp_dir, "Zsp.string_data"))?;
                    } else if filename_str.ends_with(".string_index") {
                        rename(path, filepath(&zsp_dir, "Zsp.string_index"))?;
                    } else {
                        panic!("unexpected file {:?}", path);
                    }
                }
                assert_eq!(num_files, 3);

                // Run `summarize`.
                let mut summarize_cmd = Command::new("summarize");
                summarize_cmd.arg("summarize").arg(&zsp_files_prefix);
                fs::write(
                    &summarize_file,
                    &summarize_cmd.output().context("summarize")?.stdout,
                )?;

                // Run `flamegraph`.
                let mut flamegraph_cmd = Command::new("flamegraph");
                flamegraph_cmd.arg(&zsp_files_prefix);
                flamegraph_cmd.status().context("flamegraph")?;
                rename("rustc.svg", flamegraph_file)?;

                // Run `crox`.
                let mut crox_cmd = Command::new("crox");
                crox_cmd.arg(&zsp_files_prefix);
                crox_cmd.status().context("crox")?;
                rename("chrome_profiler.json", crox_file)?;
            }

            // `-Ztime-passes` output is redirected (via rustc-fake) to a file
            // called `Ztp`. We copy that output into a file in the output dir.
            Profiler::TimePasses => {
                let tmp_ztp_file = filepath(data.cwd.as_ref(), "Ztp");
                let ztp_file = filepath(self.output_dir, &out_file("Ztp"));

                fs::copy(&tmp_ztp_file, &ztp_file)?;
            }

            // perf-record produces (via rustc-fake) a data file called `perf`.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process.
            Profiler::PerfRecord => {
                let tmp_perf_file = filepath(data.cwd.as_ref(), "perf");
                let perf_file = filepath(self.output_dir, &out_file("perf"));

                fs::copy(&tmp_perf_file, &perf_file)?;
            }

            // OProfile produces (via rustc-fake) a data directory called
            // `oprofile_data`. We copy it from the temp dir to the output dir,
            // giving it a new name in the process, and then post-process it
            // twice to produce another two data files in the output dir.
            Profiler::OProfile => {
                let tmp_opout_dir = filepath(data.cwd.as_ref(), "oprofile_data");
                let opout_dir = filepath(self.output_dir, &out_file("opout"));
                let oprep_file = filepath(self.output_dir, &out_file("oprep"));
                let opann_file = filepath(self.output_dir, &out_file("opann"));

                // Move the directory.
                if opout_dir.exists() {
                    fs::remove_dir_all(&opout_dir)?;
                }
                rename(&tmp_opout_dir, &opout_dir)?;

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
                fs::write(oprep_file, &op_report_cmd.output()?.stdout)?;

                let mut op_annotate_cmd = Command::new("opannotate");
                // Other possibly useful args: --assembly
                op_annotate_cmd
                    .arg("--source")
                    .arg("--threshold")
                    .arg("0.5")
                    .arg(&session_dir_arg);
                fs::write(opann_file, &op_annotate_cmd.output()?.stdout)?;
            }

            // Cachegrind produces (via rustc-fake) a data file called `cgout`.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process, and then post-process it to produce another
            // data file in the output dir.
            Profiler::Cachegrind => {
                let tmp_cgout_file = filepath(data.cwd.as_ref(), "cgout");
                let cgout_file = filepath(self.output_dir, &out_file("cgout"));
                let cgann_file = filepath(self.output_dir, &out_file("cgann"));

                fs::copy(&tmp_cgout_file, &cgout_file)?;

                let mut cg_annotate_cmd = Command::new("cg_annotate");
                cg_annotate_cmd
                    .arg("--auto=yes")
                    .arg("--show-percs=yes")
                    .arg(&cgout_file);
                fs::write(cgann_file, &cg_annotate_cmd.output()?.stdout)?;
            }

            // Callgrind produces (via rustc-fake) a data file called `clgout`.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process, and then post-process it to produce another
            // data file in the output dir.
            Profiler::Callgrind => {
                let tmp_clgout_file = filepath(data.cwd.as_ref(), "clgout");
                let clgout_file = filepath(self.output_dir, &out_file("clgout"));
                let clgann_file = filepath(self.output_dir, &out_file("clgann"));

                fs::copy(&tmp_clgout_file, &clgout_file)?;

                let mut clg_annotate_cmd = Command::new("callgrind_annotate");
                clg_annotate_cmd
                    .arg("--auto=yes")
                    .arg("--show-percs=yes")
                    .arg(&clgout_file);
                fs::write(clgann_file, &clg_annotate_cmd.output()?.stdout)?;
            }

            // DHAT produces (via rustc-fake) a data file called `dhout`. We
            // copy it from the temp dir to the output dir, giving it a new
            // name in the process.
            Profiler::DHAT => {
                let tmp_dhout_file = filepath(data.cwd.as_ref(), "dhout");
                let dhout_file = filepath(self.output_dir, &out_file("dhout"));

                fs::copy(&tmp_dhout_file, &dhout_file)?;
            }

            // Massif produces (via rustc-fake) a data file called `msout`. We
            // copy it from the temp dir to the output dir, giving it a new
            // name in the process.
            Profiler::Massif => {
                let tmp_msout_file = filepath(data.cwd.as_ref(), "msout");
                let msout_file = filepath(self.output_dir, &out_file("msout"));

                fs::copy(&tmp_msout_file, &msout_file)?;
            }

            // `eprintln!` statements are redirected (via rustc-fake) to a file
            // called `eprintln`. We copy it from the temp dir to the output
            // dir, giving it a new name in the process.
            Profiler::Eprintln => {
                let tmp_eprintln_file = filepath(data.cwd.as_ref(), "eprintln");
                let eprintln_file = filepath(self.output_dir, &out_file("eprintln"));

                fs::copy(&tmp_eprintln_file, &eprintln_file)?;
            }

            // `cargo llvm-lines` writes its output to stdout. We copy that
            // output into a file in the output dir.
            Profiler::LlvmLines => {
                let ll_file = filepath(self.output_dir, &out_file("ll"));

                fs::write(ll_file, &output.stdout)?;
            }
        }
        Ok(Retry::No)
    }
}

impl Benchmark {
    pub fn new(name: String, path: PathBuf) -> anyhow::Result<Self> {
        let mut patches = vec![];
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "patch" {
                    patches.push(path.clone());
                }
            }
        }

        patches.sort();

        let patches = patches.into_iter().map(|p| Patch::new(p)).collect();

        let config_path = path.join("perf-config.json");
        let config: BenchmarkConfig = if config_path.exists() {
            serde_json::from_reader(
                File::open(&config_path)
                    .with_context(|| format!("failed to open {:?}", config_path))?,
            )
            .with_context(|| format!("failed to parse {:?}", config_path))?
        } else {
            BenchmarkConfig::default()
        };

        Ok(Benchmark {
            name: BenchmarkName(name),
            path,
            patches,
            config,
        })
    }

    pub fn supports_stable(&self) -> bool {
        self.config.supports_stable
    }

    fn make_temp_dir(&self, base: &Path) -> anyhow::Result<TempDir> {
        // Appending `.` means we copy just the contents of `base` into
        // `tmp_dir`, rather than `base` itself.
        let mut base_dot = base.to_path_buf();
        base_dot.push(".");
        let tmp_dir = TempDir::new()?;
        let mut cmd = Command::new("cp");
        cmd.arg("-pLR").arg(base_dot).arg(tmp_dir.path());
        command_output(&mut cmd).with_context(|| format!("copying {} to tmp dir", self.name))?;
        Ok(tmp_dir)
    }

    fn mk_cargo_process<'a>(
        &'a self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        build_kind: BuildKind,
    ) -> CargoProcess<'a> {
        let mut cargo_args = self
            .config
            .cargo_opts
            .clone()
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        if let Some(count) = env::var("CARGO_THREAD_COUNT")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
        {
            cargo_args.push(format!("-j{}", count));
        }

        CargoProcess {
            compiler,
            processor_name: self.name.clone(),
            cwd,
            build_kind,
            incremental: false,
            processor_etc: None,
            manifest_path: self
                .config
                .cargo_toml
                .clone()
                .unwrap_or_else(|| String::from("Cargo.toml")),
            cargo_args,
            rustc_args: self
                .config
                .cargo_rustc_opts
                .clone()
                .unwrap_or_default()
                .split_whitespace()
                .map(String::from)
                .collect(),
            touch_file: self.config.touch_file.clone(),
            jobserver: None,
        }
    }

    /// Run a specific benchmark under a processor + profiler combination.
    pub fn measure(
        &self,
        processor: &mut dyn Processor,
        build_kinds: &[BuildKind],
        run_kinds: &[RunKind],
        compiler: Compiler<'_>,
        iterations: usize,
    ) -> anyhow::Result<()> {
        let iterations = cmp::min(iterations, self.config.runs);

        if self.config.disabled || build_kinds.is_empty() {
            eprintln!("Skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        eprintln!("Preparing {}", self.name);
        let build_kind_dirs = build_kinds
            .iter()
            .map(|kind| Ok((*kind, self.make_temp_dir(&self.path)?)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        // In parallel (but with a limit to the number of CPUs), prepare all
        // builds kinds. This is done in parallel vs. sequentially because:
        //  * We don't record any measurements during this phase, so the
        //    performance need not be consistent.
        //  * We want to make use of the reality that rustc is single-threaded
        //    during a good portion of compilation; that means that it is faster
        //    to run this preparation when we can interleave rustc's as needed
        //    rather than fully sequentially, where we have long periods of a
        //    single CPU core being used.
        //
        // As one example, with a full (All builds x All run kinds)
        // configuration, script-servo-2 took 2995s without this parallelization
        // and 2915s with. This is a small win, admittedly, but even a few
        // minutes shaved off is important -- and there's not too much mangling
        // of our code needed to get this to work. This benchmark has since been
        // deleted, but the optimization holds for other crates as well.
        //
        // Ideally we would not separately build build-script's (which are
        // otherwise shared between the configurations), but there's no good way
        // to do this in Cargo today. We would also ideally build in the same
        // target directory, but that's also not possible, as Cargo takes a
        // target-directory global lock during compilation.
        crossbeam_utils::thread::scope::<_, anyhow::Result<()>>(|s| {
            let server = jobserver::Client::new(num_cpus::get()).context("jobserver::new")?;
            for (build_kind, prep_dir) in &build_kind_dirs {
                let server = server.clone();
                s.spawn::<_, anyhow::Result<()>>(move |_| {
                    self.mk_cargo_process(compiler, prep_dir.path(), *build_kind)
                        .jobserver(server)
                        .run_rustc(false)?;
                    Ok(())
                });
            }
            Ok(())
        })
        .unwrap()?;

        for (build_kind, prep_dir) in build_kind_dirs {
            eprintln!("Running {}: {:?} + {:?}", self.name, build_kind, run_kinds);

            // We want at least two runs for all benchmarks (since we run
            // self-profile separately).
            processor.start_first_collection();
            for i in 0..cmp::max(iterations, 2) {
                if i == 1 {
                    let different = processor.finished_first_collection(build_kind);
                    if iterations == 1 && !different {
                        // Don't run twice if this processor doesn't need it and
                        // we've only been asked to run once.
                        break;
                    }
                }
                log::debug!("Benchmark iteration {}/{}", i + 1, iterations);
                let timing_dir = self.make_temp_dir(prep_dir.path())?;
                let cwd = timing_dir.path();

                // A full non-incremental build.
                if run_kinds.contains(&RunKind::Full) {
                    self.mk_cargo_process(compiler, cwd, build_kind)
                        .processor(processor, RunKind::Full, "Full", None)
                        .run_rustc(true)?;
                }

                // Rustdoc does not support incremental compilation
                if build_kind != BuildKind::Doc {
                    // An incremental build from scratch (slowest incremental case).
                    // This is required for any subsequent incremental builds.
                    if run_kinds.contains(&RunKind::IncrFull)
                        || run_kinds.contains(&RunKind::IncrUnchanged)
                        || run_kinds.contains(&RunKind::IncrPatched)
                    {
                        self.mk_cargo_process(compiler, cwd, build_kind)
                            .incremental(true)
                            .processor(processor, RunKind::IncrFull, "IncrFull", None)
                            .run_rustc(true)?;
                    }

                    // An incremental build with no changes (fastest incremental case).
                    if run_kinds.contains(&RunKind::IncrUnchanged) {
                        self.mk_cargo_process(compiler, cwd, build_kind)
                            .incremental(true)
                            .processor(processor, RunKind::IncrUnchanged, "IncrUnchanged", None)
                            .run_rustc(true)?;
                    }

                    if run_kinds.contains(&RunKind::IncrPatched) {
                        for (i, patch) in self.patches.iter().enumerate() {
                            log::debug!("applying patch {}", patch.name);
                            patch.apply(cwd).map_err(|s| anyhow::anyhow!("{}", s))?;

                            // An incremental build with some changes (realistic
                            // incremental case).
                            let run_kind_str = format!("IncrPatched{}", i);
                            self.mk_cargo_process(compiler, cwd, build_kind)
                                .incremental(true)
                                .processor(
                                    processor,
                                    RunKind::IncrPatched,
                                    &run_kind_str,
                                    Some(&patch),
                                )
                                .run_rustc(true)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, PartialEq, Eq, Debug)]
enum DeserializeStatError {
    #[error("could not deserialize empty output to stats, output: {:?}", .0)]
    NoOutput(process::Output),
    #[error("could not parse `{}` as a float", .0)]
    ParseError(String, #[source] ::std::num::ParseFloatError),
}

struct SelfProfileFiles {
    string_data: PathBuf,
    string_index: PathBuf,
    events: PathBuf,
}

fn process_perf_stat_output(
    output: process::Output,
) -> Result<(Stats, Option<SelfProfile>, Option<SelfProfileFiles>), DeserializeStatError> {
    let stdout = String::from_utf8(output.stdout.clone()).expect("utf8 output");
    let mut stats = Stats::new();

    let mut profile: Option<SelfProfile> = None;
    let mut dir: Option<PathBuf> = None;
    let mut prefix: Option<String> = None;
    for line in stdout.lines() {
        if line.starts_with("!self-profile-output:") {
            profile = Some(serde_json::from_str(&line["!self-profile-output:".len()..]).unwrap());
            continue;
        }
        if line.starts_with("!self-profile-dir:") {
            dir = Some(PathBuf::from(&line["!self-profile-dir:".len()..]));
            continue;
        }
        if line.starts_with("!self-profile-prefix:") {
            prefix = Some(String::from(&line["!self-profile-prefix:".len()..]));
            continue;
        }

        // github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e: expr) => {
                match $e {
                    Some(s) => s,
                    None => {
                        log::warn!("unhandled line: {}", line);
                        continue;
                    }
                }
            };
        }
        let mut parts = line.split(';').map(|s| s.trim());
        let cnt = get!(parts.next());
        let _unit = get!(parts.next());
        let name = get!(parts.next());
        let _time = get!(parts.next());
        let pct = get!(parts.next());
        if cnt == "<not supported>" || cnt.len() == 0 {
            continue;
        }
        if !pct.starts_with("100.") {
            panic!(
                "measurement of `{}` only active for {}% of the time",
                name, pct
            );
        }
        stats.insert(
            name.to_owned(),
            cnt.parse()
                .map_err(|e| DeserializeStatError::ParseError(cnt.to_string(), e))?,
        );
    }

    let files = if let (Some(prefix), Some(dir)) = (prefix, dir) {
        let mut files = SelfProfileFiles {
            string_index: PathBuf::new(),
            string_data: PathBuf::new(),
            events: PathBuf::new(),
        };
        // Rename the data files. There should be exactly three.
        for entry in fs::read_dir(&dir).unwrap() {
            let filename = entry.unwrap().file_name();
            let filename_str = filename.to_str().unwrap();
            let path = dir.join(filename_str);
            if filename_str.ends_with(".events") {
                assert!(filename_str.contains(&prefix), "{:?}", path);
                files.events = path;
            } else if filename_str.ends_with(".string_data") {
                assert!(filename_str.contains(&prefix), "{:?}", path);
                files.string_data = path;
            } else if filename_str.ends_with(".string_index") {
                assert!(filename_str.contains(&prefix), "{:?}", path);
                files.string_index = path;
            }
        }

        Some(files)
    } else {
        None
    };

    if stats.is_empty() {
        return Err(DeserializeStatError::NoOutput(output));
    }

    Ok((stats, profile, files))
}

#[derive(Clone)]
pub struct Stats {
    stats: HashMap<String, f64>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> + '_ {
        self.stats.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }

    pub fn insert(&mut self, stat: String, value: f64) {
        self.stats.insert(stat, value);
    }
}

#[derive(Debug, Clone)]
pub struct Patch {
    index: usize,
    pub name: PatchName,
    path: PathBuf,
}

impl PartialEq for Patch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Patch {}

impl hash::Hash for Patch {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.name.hash(h);
    }
}

impl Patch {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        let (index, name) = {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let mut parts = file_name.split("-");
            let index = parts.next().unwrap().parse().unwrap_or_else(|e| {
                panic!(
                    "{:?} should be in the format 000-name.patch, \
                     but did not start with a number: {:?}",
                    &path, e
                );
            });
            let mut name = parts.fold(String::new(), |mut acc, part| {
                acc.push_str(part);
                acc.push(' ');
                acc
            });
            let len = name.len();
            // take final space off
            name.truncate(len - 1);
            let name = name.replace(".patch", "");
            (index, name)
        };

        Patch {
            path: PathBuf::from(path.file_name().unwrap().to_str().unwrap()),
            index,
            name: name.as_str().into(),
        }
    }

    pub fn apply(&self, dir: &Path) -> Result<(), String> {
        log::debug!("applying {} to {:?}", self.name, dir);
        let mut cmd = Command::new("patch");
        cmd.current_dir(dir).args(&["-Np1", "-i"]).arg(&*self.path);
        cmd.stdout(Stdio::null());
        if cmd.status().map(|s| !s.success()).unwrap_or(false) {
            return Err(format!("could not execute {:?}.", cmd));
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

#[derive(serde::Deserialize, Clone)]
pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: Duration,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
}
