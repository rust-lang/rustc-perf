use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute::SelfProfileFiles;
use analyzeme::ProfilingData;
use anyhow::Context;
use database::{
    ArtifactId, ArtifactIdNumber, CodegenBackend, CollectionId, Parallel, Profile, Scenario, Target,
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
        parallel: Parallel,
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
            //  self-profile/<artifact id>/<benchmark>/<target>/<backend>/<profile>/<scenario>/<parallel>
            //    /self-profile.mm_profdata.sz
            SelfProfileId::Simple {
                artifact_id,
                benchmark,
                profile,
                scenario,
                parallel,
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
                    .join(parallel.par_n())
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
    client: object_store::aws::AmazonS3,
}

#[cfg(feature = "s3-sdk")]
impl S3WriteContext {
    async fn new() -> Self {
        use std::time::Duration;

        let mut retry_config = object_store::RetryConfig::default();
        retry_config.retry_timeout = Duration::from_secs(60 * 2);
        retry_config.max_retries = 3;

        let key_id = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID is not set");
        let key_secret =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY is not set");

        let client_config = object_store::client::ClientOptions::new()
            .with_timeout(Duration::from_secs(15))
            .with_connect_timeout(Duration::from_secs(15))
            .with_pool_idle_timeout(Duration::from_secs(15));

        let client = object_store::aws::AmazonS3Builder::new()
            .with_region("us-west-1")
            .with_bucket_name("rustc-perf")
            .with_retry(retry_config)
            .with_client_options(client_config)
            .with_secret_access_key(key_secret)
            .with_access_key_id(key_id)
            .build()
            .expect("Could not build S3 client");
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
            use object_store::ObjectStore;

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
                let mut attributes = object_store::Attributes::new();
                attributes.insert(
                    object_store::Attribute::StorageClass,
                    "INTELLIGENT_TIERING".into(),
                );
                ctx.client
                    .put_opts(
                        &object_store::path::Path::parse(file_path.to_str().unwrap()).unwrap(),
                        object_store::PutPayload::from(compressed),
                        object_store::PutOptions {
                            mode: object_store::PutMode::Overwrite,
                            tags: Default::default(),
                            attributes,
                            extensions: Default::default(),
                        },
                    )
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
