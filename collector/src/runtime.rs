use crate::benchmark::profile::Profile;
use crate::toolchain::{get_local_toolchain, LocalToolchain};
use benchlib::messages::BenchmarkResult;
use cargo_metadata::Message;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct BenchmarkBinary {
    path: PathBuf,
}

pub fn bench_runtime(rustc: &str, id: Option<&str>, benchmark_dir: PathBuf) -> anyhow::Result<()> {
    let toolchain = get_local_toolchain(&[Profile::Opt], rustc, None, None, id, "")?;
    let output = compile_runtime_benchmarks(&toolchain, &benchmark_dir)?;
    let binaries = gather_binaries(&output)?;

    for binary in binaries {
        let name = binary.path.file_name().and_then(|s| s.to_str()).unwrap();

        let result = Command::new(&binary.path).arg("benchmark").output()?;
        if !result.status.success() {
            anyhow::bail!(
                "Failed to run runtime benchmark {name}\n{}\n{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
        } else {
            log::info!("Successfully ran runtime benchmark {name}",);

            let data: Vec<BenchmarkResult> = serde_json::from_slice(&result.stdout)?;
            // TODO: do something with the result
            println!("{name}: {:?}", data);
        }
    }

    Ok(())
}

/// Compiles all runtime benchmarks and returns the stdout output of Cargo.
fn compile_runtime_benchmarks(toolchain: &LocalToolchain, dir: &Path) -> anyhow::Result<Vec<u8>> {
    let result = Command::new(&toolchain.cargo)
        .env("RUSTC", &toolchain.rustc)
        .arg("build")
        .arg("--release")
        .arg("--message-format")
        .arg("json")
        .current_dir(dir)
        .output()?;

    if !result.status.success() {
        anyhow::bail!(
            "Failed to compile runtime benchmarks\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    } else {
        log::info!("Successfully compiled runtime benchmarks");
        return Ok(result.stdout);
    }
}

/// Parse Cargo JSON output and find all compiled binaries.
fn gather_binaries(cargo_stdout: &[u8]) -> anyhow::Result<Vec<BenchmarkBinary>> {
    let mut binaries = vec![];

    for message in Message::parse_stream(cargo_stdout) {
        let message = message?;
        match message {
            Message::CompilerArtifact(artifact) => {
                if let Some(ref executable) = artifact.executable {
                    if artifact.target.kind.iter().any(|k| k == "bin") {
                        let path = executable.as_std_path().to_path_buf();
                        binaries.push(BenchmarkBinary { path });
                    }
                }
            }
            _ => {}
        }
    }

    log::debug!("Found binaries: {:?}", binaries);

    Ok(binaries)
}
