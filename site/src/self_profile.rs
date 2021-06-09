//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use crate::load::InputData;
use anyhow::Context;
use bytes::Buf;
use hyper::StatusCode;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;

type Response = http::Response<hyper::Body>;

pub mod crox;
pub mod flamegraph;

pub struct Output {
    pub data: Vec<u8>,
    pub filename: &'static str,
    pub is_download: bool,
}

pub fn generate(
    title: &str,
    pieces: Pieces,
    mut params: HashMap<String, String>,
) -> anyhow::Result<Output> {
    let removed = params.remove("type");
    match removed.as_deref() {
        Some("crox") => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("crox opts")?;
            Ok(Output {
                filename: "chrome_profiler.json",
                data: crox::generate(pieces, opt).context("crox")?,
                is_download: true,
            })
        }
        Some("flamegraph") => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("flame opts")?;
            Ok(Output {
                filename: "flamegraph.svg",
                data: flamegraph::generate(title, pieces, opt).context("flame")?,
                is_download: false,
            })
        }
        _ => anyhow::bail!("Unknown type, specify type={crox,flamegraph}"),
    }
}

pub struct Pieces {
    pub string_data: Vec<u8>,
    pub string_index: Vec<u8>,
    pub events: Vec<u8>,
}

impl fmt::Debug for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pieces")
            .field("string_data", &self.string_data.len())
            .field("string_index", &self.string_index.len())
            .field("events", &self.events.len())
            .finish()
    }
}

pub async fn get_pieces(
    body: crate::api::self_profile_raw::Request,
    data: &InputData,
) -> Result<Pieces, Response> {
    let res = crate::server::handle_self_profile_raw(body, data).await;
    let url = match res {
        Ok(v) => v.url,
        Err(e) => {
            let mut resp = Response::new(e.into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            return Err(resp);
        }
    };
    log::trace!("downloading {}", url);

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            let mut resp = Response::new(format!("{:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };

    if !resp.status().is_success() {
        let mut resp =
            Response::new(format!("upstream status {:?} is not successful", resp.status()).into());
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return Err(resp);
    }

    let tarball = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            let mut resp =
                Response::new(format!("could not download from upstream: {:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };
    let tarball = tar::Archive::new(std::io::BufReader::new(snap::read::FrameDecoder::new(
        tarball.reader(),
    )));
    let pieces = match Pieces::from_tarball(tarball) {
        Ok(v) => v,
        Err(e) => {
            let mut resp = Response::new(format!("could not extract from tarball: {:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };
    Ok(pieces)
}

impl Pieces {
    fn from_tarball<R: std::io::Read>(mut tarball: tar::Archive<R>) -> anyhow::Result<Pieces> {
        let mut pieces = Pieces {
            string_data: Vec::new(),
            string_index: Vec::new(),
            events: Vec::new(),
        };

        for entry in tarball.entries().context("entries")? {
            let mut entry = entry.context("tarball entry")?;
            let path = entry.path_bytes();
            if *path == *b"self-profile.string_index" {
                entry
                    .read_to_end(&mut pieces.string_index)
                    .context("reading string index")?;
            } else if *path == *b"self-profile.string_data" {
                entry
                    .read_to_end(&mut pieces.string_data)
                    .context("reading string data")?;
            } else if *path == *b"self-profile.events" {
                entry
                    .read_to_end(&mut pieces.events)
                    .context("reading events")?;
            }
        }

        Ok(pieces)
    }
}
