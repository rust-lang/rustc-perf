use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute::SelfProfileFiles;
use anyhow::Context;
use database::{ArtifactIdNumber, CollectionId, Profile, Scenario};
use std::future::Future;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::pin::Pin;

// TODO: Record codegen backend in the self profile name
// TODO: remove collection ID from self-profile ID
/// Uniquely identifies a self-profile archive.
#[derive(Debug)]
pub struct SelfProfileId {
    pub artifact_id_number: ArtifactIdNumber,
    pub collection: CollectionId,
    pub benchmark: BenchmarkName,
    pub profile: Profile,
    pub scenario: Scenario,
}

impl SelfProfileId {
    fn file_prefix(&self) -> PathBuf {
        PathBuf::from("self-profile")
            .join(self.artifact_id_number.0.to_string())
            .join(self.benchmark.0.as_str())
            .join(self.profile.to_string())
            .join(self.scenario.to_id())
    }
}

pub trait SelfProfileStorage {
    /// Store a self-profile with the given ID.
    fn store(
        &self,
        id: SelfProfileId,
        files: SelfProfileFiles,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;
}

pub struct LocalSelfProfileStorage {
    directory: PathBuf,
}

impl LocalSelfProfileStorage {
    pub fn new(dir: &Path) -> Self {
        Self {
            directory: dir.to_owned(),
        }
    }
}

impl SelfProfileStorage for LocalSelfProfileStorage {
    fn store(
        &self,
        id: SelfProfileId,
        files: SelfProfileFiles,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        let prefix = id.file_prefix();
        let path = self
            .directory
            .join(prefix)
            .join(format!("self-profile-{}.mm_profdata.sz", id.collection));
        Box::pin(async move {
            tokio::fs::create_dir_all(path.parent().unwrap()).await?;
            match files {
                SelfProfileFiles::Eight { file } => {
                    tokio::fs::copy(&file, &path).await?;
                }
            }

            Ok(())
        })
    }
}

#[derive(Default)]
pub struct S3SelfProfileStorage;

impl S3SelfProfileStorage {
    pub fn new() -> Self {
        Self
    }
}

impl SelfProfileStorage for S3SelfProfileStorage {
    fn store(
        &self,
        id: SelfProfileId,
        files: SelfProfileFiles,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        Box::pin(async move {
            // Files are placed at
            //  * self-profile/<artifact id>/<benchmark>/<profile>/<scenario>
            //    /self-profile-<collection-id>.{extension}
            let prefix = id.file_prefix();
            let upload = tempfile::NamedTempFile::new().context("cannot create temporary file")?;
            let filename = match files {
                SelfProfileFiles::Eight { file } => {
                    let data = tokio::fs::read(file)
                        .await
                        .context("cannot read self-profile data")?;
                    let mut data = snap::read::FrameEncoder::new(&data[..]);
                    let mut compressed = Vec::new();
                    data.read_to_end(&mut compressed)
                        .context("cannot compress self-profile data")?;
                    tokio::fs::write(upload.path(), &compressed)
                        .await
                        .context("cannot write compressed self-profile data")?;

                    format!("self-profile-{}.mm_profdata.sz", id.collection)
                }
            };

            let output = tokio::process::Command::new("aws")
                .arg("s3")
                .arg("cp")
                .arg("--storage-class")
                .arg("INTELLIGENT_TIERING")
                .arg("--only-show-errors")
                .arg(upload.path())
                .arg(format!(
                    "s3://rustc-perf/{}",
                    &prefix.join(filename).to_str().unwrap()
                ))
                .spawn()
                .context("cannot spawn aws binary")?
                .wait_with_output()
                .await
                .context("cannot run aws binary")?;
            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "Could not upload self-profile to S3\nStdout:\n{}\nStderr:\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Ok(())
        })
    }
}
