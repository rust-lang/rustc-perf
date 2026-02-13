use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute::SelfProfileFiles;
use analyzeme::ProfilingData;
use anyhow::Context;
use database::{
    ArtifactId, ArtifactIdNumber, CodegenBackend, CollectionId, Profile, Scenario, Target,
};
use reqwest::StatusCode;
use std::future::Future;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::Instant;

/// Uniquely identifies a self-profile archive.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum SelfProfileId {
    /// Legacy ID with artifact ID number and collection ID
    /// Exists for backwards compatibility with old self-profile links.
    /// TODO: remove in Q3 2026
    Legacy {
        artifact_id_number: ArtifactIdNumber,
        collection: CollectionId,
        benchmark: BenchmarkName,
        profile: Profile,
        scenario: Scenario,
    },
    /// Simplified ID with artifact name and without collection ID
    Simple {
        artifact_id: ArtifactId,
        benchmark: BenchmarkName,
        profile: Profile,
        scenario: Scenario,
        backend: CodegenBackend,
        target: Target,
    },
}

impl SelfProfileId {
    fn relative_file_path(&self) -> PathBuf {
        match self {
            //  self-profile/<artifact id number>/<benchmark>/<profile>/<scenario>
            //    /self-profile-<collection-id>.mm_profdata.sz
            SelfProfileId::Legacy {
                profile,
                benchmark,
                collection,
                artifact_id_number,
                scenario,
            } => PathBuf::from("self-profile")
                .join(artifact_id_number.0.to_string())
                .join(benchmark.0.as_str())
                .join(profile.to_string())
                .join(scenario.to_id())
                .join(format!("self-profile-{collection}.mm_profdata.sz")),
            //  self-profile/<artifact id>/<benchmark>/<target>/<backend>/<profile>/<scenario>
            //    /self-profile.mm_profdata.sz
            SelfProfileId::Simple {
                artifact_id,
                benchmark,
                profile,
                scenario,
                backend: codegen_backend,
                target,
            } => {
                let artifact_name = match artifact_id {
                    ArtifactId::Commit(c) => &c.sha,
                    ArtifactId::Tag(name) => name,
                };
                PathBuf::from("self-profile")
                    .join(artifact_name)
                    .join(benchmark.0.as_str())
                    .join(target.to_string())
                    .join(codegen_backend.to_string())
                    .join(profile.to_string())
                    .join(scenario.to_id())
                    .join("self-profile.mm_profdata.sz")
            }
        }
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
        self.directory.join(id.relative_file_path())
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

/// There seems to be a non-trivial cost (1-2s) for initiating a new S3 connection.
/// If we initiate a new connection for every uploaded file, that can quickly add up.
/// We thus cache the client, which makes the follow-up uploads much faster.
#[cfg(feature = "s3-sdk")]
#[derive(Clone)]
struct S3WriteContext {
    client: aws_sdk_s3::Client,
}

#[cfg(feature = "s3-sdk")]
impl S3WriteContext {
    async fn new() -> Self {
        use aws_sdk_s3::config::retry::RetryConfig;
        use aws_sdk_s3::config::timeout::TimeoutConfig;
        use std::time::Duration;

        let region = aws_config::Region::new("us-west-1");
        let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region)
            .retry_config(RetryConfig::standard().with_max_attempts(3))
            .timeout_config(
                TimeoutConfig::builder()
                    .connect_timeout(Duration::from_secs(15))
                    .read_timeout(Duration::from_secs(15))
                    .operation_attempt_timeout(Duration::from_secs(60))
                    .build(),
            )
            .load()
            .await;
        let client = aws_sdk_s3::Client::new(&shared_config);
        Self { client }
    }
}

pub struct S3SelfProfileStorage {
    #[cfg(feature = "s3-sdk")]
    write_ctx: S3WriteContext,
}

impl S3SelfProfileStorage {
    pub async fn new() -> Self {
        Self {
            #[cfg(feature = "s3-sdk")]
            write_ctx: S3WriteContext::new().await,
        }
    }
}

impl SelfProfileStorage for S3SelfProfileStorage {
    fn store(
        &self,
        _id: SelfProfileId,
        _files: SelfProfileFiles,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        #[cfg(feature = "s3-sdk")]
        {
            let ctx = self.write_ctx.clone();
            Box::pin(async move {
                let file_path = _id.relative_file_path();
                let compressed = match _files {
                    SelfProfileFiles::Eight { file } => {
                        let start = Instant::now();
                        let data = tokio::fs::read(file)
                            .await
                            .context("cannot read self-profile data")?;
                        log::trace!(
                            "Read self-profile duration: {}, size: {}",
                            start.elapsed().as_secs_f64(),
                            data.len()
                        );
                        let start = Instant::now();

                        // This is synchronous and blocks the event loop, so we should do it on a
                        // worker thread
                        let compressed = tokio::task::spawn_blocking(move || {
                            let mut data = snap::read::FrameEncoder::new(&data[..]);
                            let mut compressed = Vec::new();

                            data.read_to_end(&mut compressed)
                                .context("cannot compress self-profile data")?;
                            anyhow::Ok(compressed)
                        })
                        .await??;

                        log::trace!(
                            "Compress self-profile duration: {}, size: {}",
                            start.elapsed().as_secs_f64(),
                            compressed.len()
                        );
                        compressed
                    }
                };

                log::info!("Uploading self-profile to {}", file_path.display());

                let start = Instant::now();
                let body = aws_sdk_s3::primitives::ByteStream::from_body_1_x(
                    aws_sdk_s3::primitives::SdkBody::from(compressed),
                );
                ctx.client
                    .put_object()
                    .bucket("rustc-perf")
                    .key(
                        file_path
                            .to_str()
                            .expect("Non UTF-8 path used for self-profile"),
                    )
                    .storage_class(aws_sdk_s3::types::StorageClass::IntelligentTiering)
                    .body(body)
                    .send()
                    .await
                    .context("s3 upload failed")?;
                log::trace!(
                    "Upload self-profile duration: {}",
                    start.elapsed().as_secs_f64()
                );
                Ok(())
            })
        }
        #[cfg(not(feature = "s3-sdk"))]
        panic!("S3 upload is not enabled, compile `collector` with the `s3-sdk` feature enabled.");
    }

    fn load_raw(
        &self,
        id: SelfProfileId,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Vec<u8>>>> + Send>> {
        let path = id.relative_file_path();
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
