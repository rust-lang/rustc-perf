use chrono::NaiveDate;
pub use database::{Commit, PatchName, QueryLabel};
use serde::Deserialize;
use std::cmp::PartialOrd;
use std::fmt;
use std::process::{self, Command};

pub mod api;
pub mod artifact_stats;
pub mod cargo;
pub mod codegen;
pub mod compile;
pub mod runtime;
pub mod toolchain;
pub mod utils;

use crate::compile::benchmark::{Benchmark, BenchmarkName};
use crate::runtime::{BenchmarkGroup, BenchmarkSuite};
use database::{ArtifactId, ArtifactIdNumber, Connection};
use process::Stdio;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct DeltaTime(#[serde(with = "round_float")] pub f64);

/// The bound for finding an artifact
///
/// This can either be the upper or lower bound.
/// In the case of commits or tags this is an exact bound, but for dates
/// it's a best effort (i.e., if the bound is a date but there are no artifacts
/// for that date, we'll find the artifact that most closely matches).
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    /// An unverified git commit (in sha form) or a tag of a commit (e.g., "1.53.0")
    Commit(String),
    /// A date in time
    Date(NaiveDate),
    /// No bound
    #[default]
    None,
}

impl Bound {
    /// Tests whether `self` matches commit when searching from the left
    pub fn left_match(&self, commit: &Commit) -> bool {
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.is_master() && commit.date.0.naive_utc().date() >= *date,
            Bound::None => {
                let last_month = chrono::Utc::now().date_naive() - chrono::Duration::days(30);
                commit.is_master() && last_month <= commit.date.0.naive_utc().date()
            }
        }
    }

    /// Tests whether `self` matches commit when searching from the right
    pub fn right_match(&self, commit: &Commit) -> bool {
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.is_master() && commit.date.0.date_naive() <= *date,
            Bound::None => commit.is_master(),
        }
    }
}

impl serde::Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let s = match *self {
            Bound::Commit(ref s) => s.clone(),
            Bound::Date(ref date) => date.format("%Y-%m-%d").to_string(),
            Bound::None => String::new(),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Bound {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bound, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct BoundVisitor;

        impl<'de> serde::de::Visitor<'de> for BoundVisitor {
            type Value = Bound;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Bound, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    return Ok(Bound::None);
                }

                let bound = value
                    .parse::<chrono::NaiveDate>()
                    .map(Bound::Date)
                    .unwrap_or(Bound::Commit(value.to_string()));
                Ok(bound)
            }

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("either a YYYY-mm-dd date or a collection ID (usually commit hash)")
            }
        }

        deserializer.deserialize_str(BoundVisitor)
    }
}

pub fn null_means_nan<'de, D>(deserializer: D) -> ::std::result::Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or(0.0))
}

pub fn version_supports_doc(version_str: &str) -> bool {
    if let Ok(version) = version_str.parse::<semver::Version>() {
        version >= semver::Version::new(1, 46, 0)
    } else {
        assert!(version_str.starts_with("beta") || version_str.starts_with("master"));
        true
    }
}

pub fn version_supports_incremental(version_str: &str) -> bool {
    if let Ok(version) = version_str.parse::<semver::Version>() {
        version >= semver::Version::new(1, 24, 0)
    } else {
        assert!(version_str.starts_with("beta") || version_str.starts_with("master"));
        true
    }
}

/// Rounds serialized and deserialized floats to 2 decimal places.
pub mod round_float {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(n: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64((*n * 100.0).round() / 100.0)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = f64::deserialize(deserializer)?;
        Ok((n * 100.0).round() / 100.0)
    }
}

pub fn run_command(cmd: &mut Command) -> anyhow::Result<()> {
    log::trace!("running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("expected success {:?}", status));
    }
    Ok(())
}

fn run_command_with_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    use anyhow::Context;
    use utils::read2;
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn process for cmd: {:?}", cmd))?;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut stdout_writer = std::io::LineWriter::new(std::io::stdout());
    let mut stderr_writer = std::io::LineWriter::new(std::io::stderr());
    read2::read2(
        child.stdout.take().unwrap(),
        child.stderr.take().unwrap(),
        &mut |is_stdout, buffer, _is_done| {
            // Send output if trace logging is enabled
            if log::log_enabled!(target: "raw_cargo_messages", log::Level::Trace) {
                use std::io::Write;
                if is_stdout {
                    stdout_writer.write_all(&buffer[stdout.len()..]).unwrap();
                } else {
                    stderr_writer.write_all(&buffer[stderr.len()..]).unwrap();
                }
            }
            if is_stdout {
                stdout = buffer.clone();
            } else {
                stderr = buffer.clone();
            }
        },
    )?;

    let status = child
        .wait()
        .with_context(|| "failed to wait on child process")?;

    Ok(process::Output {
        status,
        stdout,
        stderr,
    })
}

pub fn command_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    let output = run_command_with_output(cmd)?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}\n",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(output)
}

pub async fn async_command_output(
    mut cmd: tokio::process::Command,
) -> anyhow::Result<process::Output> {
    use anyhow::Context;

    let start = Instant::now();
    let child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn process for cmd: {:?}", cmd))?;
    let output = child.wait_with_output().await?;
    log::trace!("command {cmd:?} took {} ms", start.elapsed().as_millis());

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}\n",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(output)
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterCommit {
    pub sha: String,
    pub parent_sha: String,
    /// This is the pull request which this commit merged in.
    #[serde(default)]
    pub pr: Option<u32>,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// This provides the master-branch Rust commits which should have accompanying
/// bors artifacts available.
///
/// The first commit returned (at index 0) is the most recent, the last is the
/// oldest.
///
/// Specifically, this is the last 168 days of bors commits.
///
/// Note that this does not contain try commits today, so it should not be used
/// to validate hashes or expand them generally speaking. This may also change
/// in the future.
pub async fn master_commits() -> anyhow::Result<Vec<MasterCommit>> {
    let response = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(60))
        .timeout(Duration::from_secs(60))
        .build()?
        .get("https://triage.rust-lang.org/bors-commit-list")
        .send()
        .await?;
    Ok(response.json().await?)
}

/// Collects information about steps that will be benchmarked during a single artifact run.
#[derive(Default)]
pub struct CollectorStepBuilder {
    steps: Vec<String>,
}

impl CollectorStepBuilder {
    pub fn record_compile_benchmarks(
        mut self,
        benchmarks: &[Benchmark],
        bench_rustc: bool,
    ) -> Self {
        self.steps
            .extend(benchmarks.iter().map(|b| b.name.to_string()));
        if bench_rustc {
            self.steps.push("rustc".to_string());
        }
        self
    }

    pub fn record_runtime_benchmarks(mut self, suite: &BenchmarkSuite) -> Self {
        self.steps.extend(
            suite
                .groups
                .iter()
                .map(|group| runtime_group_step_name(&group.name)),
        );
        self
    }

    pub async fn start_collection(
        self,
        conn: &mut dyn Connection,
        artifact_id: &ArtifactId,
    ) -> CollectorCtx {
        // Make sure there is no observable time when the artifact ID is available
        // but the in-progress steps are not.
        let artifact_row_id = {
            let mut tx = conn.transaction().await;
            let artifact_row_id = tx.conn().artifact_id(artifact_id).await;
            tx.conn()
                .collector_start(artifact_row_id, &self.steps)
                .await;
            tx.commit().await.unwrap();
            artifact_row_id
        };
        CollectorCtx { artifact_row_id }
    }
}

/// Represents an in-progress run for a given artifact.
pub struct CollectorCtx {
    pub artifact_row_id: ArtifactIdNumber,
}

impl CollectorCtx {
    pub async fn start_compile_step(
        &self,
        conn: &dyn Connection,
        benchmark_name: &BenchmarkName,
    ) -> bool {
        conn.collector_start_step(self.artifact_row_id, &benchmark_name.0)
            .await
    }

    pub async fn end_compile_step(&self, conn: &dyn Connection, benchmark_name: &BenchmarkName) {
        conn.collector_end_step(self.artifact_row_id, &benchmark_name.0)
            .await
    }

    /// Starts a new runtime benchmark collector step.
    /// If this step was already computed, returns None.
    /// Otherwise returns Some(<name of step>).
    pub async fn start_runtime_step(
        &self,
        conn: &dyn Connection,
        group: &BenchmarkGroup,
    ) -> Option<String> {
        let step_name = runtime_group_step_name(&group.name);
        conn.collector_start_step(self.artifact_row_id, &step_name)
            .await
            .then_some(step_name)
    }

    pub async fn end_runtime_step(&self, conn: &dyn Connection, group: &BenchmarkGroup) {
        conn.collector_end_step(self.artifact_row_id, &runtime_group_step_name(&group.name))
            .await
    }
}

pub fn runtime_group_step_name(benchmark_name: &str) -> String {
    format!("runtime:{}", benchmark_name)
}
