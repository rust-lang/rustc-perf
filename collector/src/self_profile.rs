use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute::SelfProfileFiles;
use analyzeme::ProfilingData;
use anyhow::Context;
use database::{ArtifactIdNumber, CodegenBackend, CollectionId, Profile, Scenario, Target};
use reqwest::StatusCode;
use std::future::Future;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::Instant;

// TODO: remove collection ID from self-profile ID
/// Uniquely identifies a self-profile archive.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SelfProfileId {
    pub artifact_id_number: ArtifactIdNumber,
    pub collection: CollectionId,
    pub benchmark: BenchmarkName,
    pub profile: Profile,
    pub scenario: Scenario,
    pub codegen_backend: CodegenBackend,
    pub target: Target,
}

impl SelfProfileId {
    fn file_prefix(&self) -> PathBuf {
        PathBuf::from("self-profile")
            .join(self.artifact_id_number.0.to_string())
            .join(self.benchmark.0.as_str())
            .join(self.target.to_string())
            .join(self.codegen_backend.to_string())
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

    /// Load self-profile data with the given ID.
    /// Returns `None` if data for the ID was not found.
    fn load(
        &self,
        id: SelfProfileId,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<analyzeme::ProfilingData>>> + Send>>
    {
        let fut = self.load_raw(id);
        Box::pin(async move {
            let Some(data) = fut.await? else {
                return Ok(None);
            };
            Ok(Some(ProfilingData::from_paged_buffer(data, None).map_err(
                |e| anyhow::anyhow!("Cannot parse self-profile data: {e}"),
            )?))
        })
    }

    /// Load the raw byte data of the self-profile with the given ID.
    /// Returns `None` if data for the ID was not found.
    #[allow(clippy::type_complexity)]
    fn load_raw(
        &self,
        id: SelfProfileId,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Vec<u8>>>> + Send>>;
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

    /// Create local self-profile storage located at a local path on disk.
    pub fn default_path() -> Self {
        Self::new(Path::new("self-profile-storage"))
    }

    fn path(&self, id: &SelfProfileId) -> PathBuf {
        let prefix = id.file_prefix();
        self.directory
            .join(prefix)
            .join(format!("self-profile-{}.mm_profdata.sz", id.collection))
    }
}

impl SelfProfileStorage for LocalSelfProfileStorage {
    fn store(
        &self,
        id: SelfProfileId,
        files: SelfProfileFiles,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        let path = self.path(&id);
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

    fn load_raw(
        &self,
        id: SelfProfileId,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Vec<u8>>>> + Send>> {
        let path = self.path(&id);
        Box::pin(async move {
            if !path.is_file() {
                return Ok(None);
            }
            let data = tokio::fs::read(&path).await.with_context(|| {
                anyhow::anyhow!("Cannot read self-profile data from {}", path.display())
            })?;
            Ok(Some(data))
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

fn s3_self_profile_filename(id: &SelfProfileId) -> String {
    format!("self-profile-{}.mm_profdata.sz", id.collection)
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

                    s3_self_profile_filename(&id)
                }
            };

            log::info!("Uploading self-profile to {}/{filename}", prefix.display());

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

    fn load_raw(
        &self,
        id: SelfProfileId,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Vec<u8>>>> + Send>> {
        let path = id.file_prefix().join(s3_self_profile_filename(&id));
        let url = format!(
            "https://perf-data.rust-lang.org/{}",
            path.to_str().expect("Non UTF-8 path used for self-profile")
        );
        Box::pin(async move {
            log::trace!("Downloading {url}");
            let start = Instant::now();
            let resp = match reqwest::get(&url).await {
                Ok(r) => r,
                Err(e) => return Err(anyhow::anyhow!("{e:?}")),
            };

            if !resp.status().is_success() {
                // Hitting an unknown path is returned as forbidden
                if resp.status() == StatusCode::FORBIDDEN {
                    return Ok(None);
                }
                return Err(anyhow::anyhow!(
                    "Upstream status {:?} is not successful.\nurl={url}",
                    resp.status(),
                ));
            }

            let compressed = match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    return Err(anyhow::anyhow!("Could not download from upstream: {e:?}"));
                }
            };

            log::trace!(
                "downloaded {} bytes in {:?}",
                compressed.len(),
                start.elapsed()
            );

            // The decompression is blocking, so we should not do it in the async task directly
            let data = tokio::task::spawn_blocking(move || {
                let mut data = Vec::new();
                match snap::read::FrameDecoder::new(Cursor::new(compressed)).read_to_end(&mut data)
                {
                    Ok(_) => Ok(data),
                    Err(e) => Err(anyhow::anyhow!("Could not decode self-profile data: {e:?}")),
                }
            })
            .await??;

            Ok(Some(data))
        })
    }
}
