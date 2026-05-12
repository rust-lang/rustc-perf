use brotli::enc::BrotliEncoderParams;
use brotli::BrotliCompress;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use std::{fmt, str};

use headers::{CacheControl, ContentType, ETag, HeaderMapExt, IfNoneMatch};
use http::header::CACHE_CONTROL;
use http_body_util::{BodyExt, Full, Limited};
use hyper::body::{Bytes, Incoming};
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use log::{error, info};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

pub use crate::api::{
    self, bootstrap, comparison, dashboard, github, graphs, info, self_profile, status, triage,
    ServerResult,
};
use crate::load::{Config, SiteCtxt};
use crate::request_handlers;
use crate::resources::{Payload, ResourceResolver};

use crate::job_queue::build_queue;

pub type Request = http::Request<Incoming>;
pub type Response = http::Response<Bytes>;

macro_rules! check_http_method {
    ($lhs: expr, $rhs: expr) => {
        if $lhs != $rhs {
            return Ok(http::Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Bytes::default())
                .unwrap());
        }
    };
}

/// Server state
#[derive(Clone)]
struct Server {
    ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>,
}

impl Server {
    fn new(ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>) -> Self {
        Self { ctxt }
    }
}

#[derive(Debug, Serialize)]
struct ServerJsonError {
    error: String,
}

pub fn json_error_response(status: StatusCode, message: impl Into<String>) -> Response {
    let payload = ServerJsonError {
        error: message.into(),
    };
    let body = Bytes::from(serde_json::to_vec(&payload).unwrap());
    http::Response::builder()
        .status(status)
        .header_typed(ContentType::json())
        .body(body)
        .unwrap()
}

impl Server {
    /// Handle a synchrnous HTTP GET request
    fn handle_get<F, S>(&self, req: &Request, handler: F) -> Result<Response, ServerError>
    where
        F: FnOnce(&SiteCtxt) -> S,
        S: Serialize,
    {
        check_http_method!(*req.method(), http::Method::GET);
        let ctxt = self.ctxt.clone();
        let ctxt = ctxt.read();
        let ctxt = ctxt.as_ref().unwrap();
        let result = handler(ctxt);
        Ok(http::Response::builder()
            .header_typed(ContentType::json())
            .body(Bytes::from(serde_json::to_string(&result).unwrap()))
            .unwrap())
    }

    async fn handle_fallible_get_async<F, R, S, E>(
        &self,
        req: &Request,
        compression: &Option<BrotliEncoderParams>,
        handler: F,
    ) -> Result<Response, ServerError>
    where
        F: FnOnce(Arc<SiteCtxt>) -> R,
        R: std::future::Future<Output = Result<S, E>> + Send,
        S: Serialize,
        E: Into<Vec<u8>>,
    {
        check_http_method!(*req.method(), http::Method::GET);
        let ctxt = self.ctxt.clone();
        let ctxt = ctxt.read().as_ref().unwrap().clone();
        let result = handler(ctxt).await;
        let response = match result {
            Ok(result) => {
                let response = http::Response::builder()
                    .header_typed(ContentType::json())
                    .header_typed(CacheControl::new().with_no_cache().with_no_store());
                let body = serde_json::to_vec(&result).unwrap();
                maybe_compressed_response(response, body, compression)
            }
            Err(err) => {
                let message = String::from_utf8_lossy(&err.into()).to_string();
                json_error_response(StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };
        Ok(response)
    }

    async fn handle_metrics(&self, _req: Request) -> Response {
        use prometheus::Encoder;
        let ctxt: Arc<SiteCtxt> = self.ctxt.read().as_ref().unwrap().clone();

        let mut buffer = Vec::new();
        let r = prometheus::Registry::new();

        let queue = build_queue(ctxt.pool.connection().await.as_mut())
            .await
            .unwrap_or_default();
        let queue_length =
            prometheus::IntGauge::new("rustc_perf_queue_length", "queue length").unwrap();
        queue_length.set(queue.len() as i64);
        r.register(Box::new(queue_length)).unwrap();

        let queue_try_commits =
            prometheus::IntGauge::new("rustc_perf_queue_try_commits", "queued try commits")
                .unwrap();
        queue_try_commits.set(queue.iter().filter(|req| req.is_try()).count() as i64);
        r.register(Box::new(queue_try_commits)).unwrap();

        // Stores cache hits and misses of the self profile cache
        {
            let cache = ctxt.self_profile_cache.lock();
            let self_profile_stats = cache.get_stats();
            let self_profile_cache_hits = prometheus::IntGauge::new(
                "rustc_perf_queue_self_profile_cache_hits",
                "self profile cache hits",
            )
            .unwrap();
            self_profile_cache_hits.set(self_profile_stats.get_hits() as i64);
            r.register(Box::new(self_profile_cache_hits)).unwrap();

            let self_profile_cache_misses = prometheus::IntGauge::new(
                "rustc_perf_queue_self_profile_cache_misses",
                "self profile cache misses",
            )
            .unwrap();
            self_profile_cache_misses.set(self_profile_stats.get_misses() as i64);
            r.register(Box::new(self_profile_cache_misses)).unwrap();
        }

        let encoder = prometheus::TextEncoder::new();
        let metric_families = r.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        Response::new(buffer.into())
    }
}

#[derive(Debug)]
struct ServerError(String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "server failed: {}", self.0)
    }
}

impl std::error::Error for ServerError {}

async fn serve_req(server: Server, req: Request) -> Result<Response, ServerError> {
    // Don't attempt to get lock if we're updating
    if server.ctxt.read().is_none() {
        return Ok(Response::new(Bytes::from("no data yet, please wait")));
    }

    if req.method() == http::Method::OPTIONS {
        return Ok(http::Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Bytes::default())
            .unwrap());
    }
    let path = req.uri().path().to_owned();
    let path = path.as_str();

    let allow_compression = req
        .headers()
        .get(hyper::header::ACCEPT_ENCODING)
        .and_then(|e| e.to_str().ok())
        .is_some_and(|s| s.split(',').any(|part| part.trim().starts_with("br")));

    let compression = if allow_compression {
        // In tests on /perf/graphs and /perf/get, quality = 2 reduces size by 20-40% compared to 0,
        // while quality = 4 takes 80% longer but reduces size by less than 5% compared to 2.
        // Around 4-5 is sometimes said to be "smaller and faster than gzip".
        // [Google's default is 6](https://github.com/google/ngx_brotli#brotli_comp_level),
        // higher levels offer only small size savings but are much slower.
        Some(BrotliEncoderParams {
            quality: 2,
            ..Default::default()
        })
    } else {
        None
    };

    if let Some(response) = handle_fs_path(&req, path, allow_compression).await {
        return Ok(response);
    }

    macro_rules! check {
        ($e:expr) => {
            match $e {
                Ok(v) => v,
                Err(e) => return Ok(e),
            }
        };
    }

    match path {
        "/perf/info" => return server.handle_get(&req, request_handlers::handle_info),
        "/perf/dashboard" => {
            let query = check!(parse_query_string(req.uri()));
            return server
                .handle_fallible_get_async(&req, &compression, |c| {
                    request_handlers::handle_dashboard(query, c)
                })
                .await;
        }
        "/perf/status_page" => {
            let ctxt: Arc<SiteCtxt> = server.ctxt.read().as_ref().unwrap().clone();
            let result = request_handlers::handle_status_page(ctxt).await;
            return match result {
                Ok(result) => Ok(http::Response::builder()
                    .header_typed(ContentType::json())
                    .body(Bytes::from(serde_json::to_string(&result).unwrap()))
                    .unwrap()),
                Err(err) => Ok(http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header_typed(ContentType::text_utf8())
                    .header_typed(CacheControl::new().with_no_cache().with_no_store())
                    .body(Bytes::from(format!("{err:?}")))
                    .unwrap()),
            };
        }
        "/perf/triage" if *req.method() == http::Method::GET => {
            let ctxt: Arc<SiteCtxt> = server.ctxt.read().as_ref().unwrap().clone();
            let input: triage::Request = check!(parse_query_string(req.uri()));
            return Ok(to_triage_response(
                crate::comparison::handle_triage(input, &ctxt).await,
            ));
        }
        "/perf/compare-compile-detail-graphs" => {
            let query = check!(parse_query_string(req.uri()));
            return server
                .handle_fallible_get_async(&req, &compression, |c| {
                    request_handlers::handle_compile_detail_graphs(query, c)
                })
                .await;
        }
        "/perf/compare-compile-detail-sections" => {
            let query = check!(parse_query_string(req.uri()));
            return server
                .handle_fallible_get_async(&req, &compression, |c| {
                    request_handlers::handle_compile_detail_sections(query, c)
                })
                .await;
        }
        "/perf/compare-runtime-detail-graphs" => {
            let query = check!(parse_query_string(req.uri()));
            return server
                .handle_fallible_get_async(&req, &compression, |c| {
                    request_handlers::handle_runtime_detail_graphs(query, c)
                })
                .await;
        }
        "/perf/graphs" => {
            let query = check!(parse_query_string(req.uri()));
            return server
                .handle_fallible_get_async(&req, &compression, |c| {
                    request_handlers::handle_graphs(query, c)
                })
                .await;
        }
        "/perf/metrics" => {
            return Ok(server.handle_metrics(req).await);
        }
        "/perf/download-raw-self-profile" => {
            let ctxt: Arc<SiteCtxt> = server.ctxt.read().as_ref().unwrap().clone();
            let req = check!(parse_query_string(req.uri()));
            return Ok(request_handlers::handle_self_profile_raw_download(req, &ctxt).await);
        }
        "/perf/processed-self-profile" => {
            let ctxt: Arc<SiteCtxt> = server.ctxt.read().as_ref().unwrap().clone();
            let req = check!(parse_query_string(req.uri()));
            return Ok(request_handlers::handle_self_profile_processed_download(
                req,
                &ctxt,
                allow_compression,
            )
            .await);
        }
        _ if req.method() == http::Method::GET => return Ok(not_found()),
        _ => {}
    }

    // POST requests
    let (req, body) = req.into_parts();
    check_http_method!(req.method, http::Method::POST);
    let ctxt: Arc<SiteCtxt> = server.ctxt.read().as_ref().unwrap().clone();
    let body = match Limited::new(body, 1024 * 1024 * 10).collect().await {
        Ok(body) => body.to_bytes(),
        Err(err)
            if err
                .downcast_ref::<http_body_util::LengthLimitError>()
                .is_some() =>
        {
            return Ok(http::Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(Bytes::default())
                .unwrap())
        }
        Err(err) => return Err(ServerError(format!("failed to read body: {err}"))),
    };

    match path {
        "/perf/get" => Ok(to_response(
            crate::comparison::handle_compare(check!(parse_body(&body)), &ctxt).await,
            &compression,
        )),
        "/perf/github-hook" => {
            if !verify_gh(&ctxt.config, &req, &body) {
                return Ok(http::Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Bytes::default())
                    .unwrap());
            }
            let event = req.headers.get("X-GitHub-Event").cloned();
            let event = event.and_then(|g| g.to_str().ok().map(|s| s.to_owned()));
            let event = match event {
                Some(v) => v,
                None => {
                    return Ok(http::Response::builder()
                        .status(StatusCode::OK)
                        .body(Bytes::from("missing event header"))
                        .unwrap())
                }
            };
            match event.as_str() {
                "issue_comment" | "push" => Ok(to_response(
                    request_handlers::handle_github_webhook(
                        check!(parse_body(&body)),
                        ctxt.clone(),
                    )
                    .await,
                    &compression,
                )),
                _ => Ok(http::Response::builder()
                    .status(StatusCode::OK)
                    .body(Bytes::from(format!("unknown event: {event}")))
                    .unwrap()),
            }
        }
        "/perf/self-profile" => Ok(to_response(
            request_handlers::handle_self_profile(check!(parse_body(&body)), &ctxt).await,
            &compression,
        )),
        "/perf/bootstrap" => Ok(
            match request_handlers::handle_bootstrap(check!(parse_body(&body)), &ctxt).await {
                Ok(result) => {
                    let mut response = http::Response::builder()
                        .header_typed(ContentType::json())
                        .header_typed(CacheControl::new().with_no_cache().with_no_store());
                    response.headers_mut().unwrap().insert(
                        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        hyper::header::HeaderValue::from_static("*"),
                    );
                    let body = serde_json::to_vec(&result).unwrap();
                    response.body(Bytes::from(body)).unwrap()
                }
                Err(err) => http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header_typed(ContentType::text_utf8())
                    .header_typed(CacheControl::new().with_no_cache().with_no_store())
                    .body(Bytes::from(err))
                    .unwrap(),
            },
        ),
        _ => Ok(http::Response::builder()
            .header_typed(ContentType::html())
            .status(StatusCode::NOT_FOUND)
            .body(Bytes::default())
            .unwrap()),
    }
}

#[allow(clippy::result_large_err)]
fn parse_body<D>(body: &[u8]) -> Result<D, Response>
where
    D: DeserializeOwned,
{
    match serde_json::from_slice(body) {
        Ok(d) => Ok(d),
        Err(err) => {
            error!(
                "failed to deserialize request {}: {:?}",
                String::from_utf8_lossy(body),
                err
            );
            Err(http::Response::builder()
                .header_typed(ContentType::text_utf8())
                .status(StatusCode::BAD_REQUEST)
                .body(Bytes::from(format!(
                    "Failed to deserialize request: {err:?}"
                )))
                .unwrap())
        }
    }
}

#[allow(clippy::result_large_err)]
fn parse_query_string<D>(uri: &http::Uri) -> Result<D, Response>
where
    D: DeserializeOwned,
{
    let params: HashMap<String, String> = uri
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_default();

    match serde_json::from_str(&serde_json::to_string(&params).unwrap()) {
        Ok(d) => Ok(d),
        Err(err) => Err(http::Response::builder()
            .header_typed(ContentType::text_utf8())
            .status(StatusCode::BAD_REQUEST)
            .body(Bytes::from(format!(
                "Failed to deserialize request {uri}: {err:?}",
            )))
            .unwrap()),
    }
}

static VERSION_UUID: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4); // random UUID used as ETag for cache revalidation
static TEMPLATES: LazyLock<ResourceResolver> =
    LazyLock::new(|| ResourceResolver::new().expect("Cannot load resources"));

/// Handle the case where the path is to a static file
async fn handle_fs_path(req: &Request, path: &str, use_compression: bool) -> Option<Response> {
    if path.contains("./") | path.contains("../") {
        return Some(not_found());
    }

    let etag = ETag::from_str(&format!(r#""{}""#, *VERSION_UUID)).unwrap();
    let mut response = http::Response::builder()
        .header_typed(etag.clone())
        .header(CACHE_CONTROL, "max-age=60, stale-while-revalidate=86400"); // tell client to use cached response for one day, but revalidate in background if older than one minute

    let if_none_match = req.headers().typed_get::<IfNoneMatch>();
    if let Some(if_none_match) = if_none_match {
        if !if_none_match.precondition_passes(&etag) {
            return Some(not_modified(response)); // tell client that the resource was not modified and to use cached response
        }
    }

    async fn resolve_template(path: &str) -> Vec<u8> {
        TEMPLATES
            .get_template(&format!("pages/{path}"))
            .await
            .unwrap()
    }

    let relative_path = path.trim_start_matches('/');
    let source = match path {
        "" | "/" | "/index.html" => resolve_template("graphs.html").await,
        "/bootstrap.html"
        | "/compare.html"
        | "/dashboard.html"
        | "/detailed-query.html"
        | "/help.html"
        | "/status.html" => resolve_template(relative_path).await,
        _ => match TEMPLATES.get_static_asset(relative_path, use_compression)? {
            Payload::Compressed(data) => {
                response = response.header(
                    hyper::header::CONTENT_ENCODING,
                    hyper::header::HeaderValue::from_static("br"),
                );
                data
            }
            Payload::Uncompressed(data) => data,
        },
    };

    let p = Path::new(&path);
    match p.extension().and_then(|x| x.to_str()) {
        Some("html") => response = response.header_typed(ContentType::html()),
        Some("png") => response = response.header_typed(ContentType::png()),
        Some("json") => response = response.header_typed(ContentType::json()),
        Some("svg") => response = response.header("Content-Type", "image/svg+xml"),
        Some("css") => response = response.header("Content-Type", "text/css"),
        Some("js") => response = response.header("Content-Type", "application/javascript"),
        _ => {
            if path.is_empty() || path == "/" {
                response = response.header_typed(ContentType::html());
            }
        }
    }

    Some(response.body(Bytes::from(source)).unwrap())
}

fn not_modified(response: http::response::Builder) -> Response {
    response
        .status(StatusCode::NOT_MODIFIED)
        .body(Bytes::default())
        .unwrap()
}

fn not_found() -> Response {
    http::Response::builder()
        .header_typed(ContentType::html())
        .status(StatusCode::NOT_FOUND)
        .body(Bytes::default())
        .unwrap()
}

fn verify_gh(config: &Config, req: &http::request::Parts, body: &[u8]) -> bool {
    let gh_header = req
        .headers
        .get("X-Hub-Signature-256")
        .and_then(|g| g.to_str().ok());
    let gh_header = match gh_header {
        Some(v) => v,
        None => return false,
    };
    verify_gh_sig(config, gh_header, body).unwrap_or(false)
}

fn verify_gh_sig(cfg: &Config, header: &str, body: &[u8]) -> Option<bool> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(cfg.keys.github_webhook_secret.as_ref().unwrap().as_bytes())
            .expect("HMAC can take key of any size");
    mac.update(body);
    let sha = header.strip_prefix("sha256=")?;
    let sha = hex::decode(sha).ok()?;
    if let Ok(()) = mac.verify_slice(&sha) {
        return Some(true);
    }

    Some(false)
}

fn to_response<S>(result: ServerResult<S>, compression: &Option<BrotliEncoderParams>) -> Response
where
    S: Serialize,
{
    match result {
        Ok(result) => {
            let response = http::Response::builder()
                .header_typed(ContentType::octet_stream())
                .header_typed(CacheControl::new().with_no_cache().with_no_store());
            let body = rmp_serde::to_vec_named(&result).unwrap();
            maybe_compressed_response(response, body, compression)
        }
        Err(err) => http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header_typed(ContentType::text_utf8())
            .header_typed(CacheControl::new().with_no_cache().with_no_store())
            .body(Bytes::from(err))
            .unwrap(),
    }
}

pub fn maybe_compressed_response(
    response: http::response::Builder,
    body: Vec<u8>,
    compression: &Option<BrotliEncoderParams>,
) -> Response {
    match compression {
        None => response.body(Bytes::from(body)).unwrap(),
        Some(brotli_params) => {
            let compressed = compress_bytes(&body, brotli_params);
            let response = response.header(
                hyper::header::CONTENT_ENCODING,
                hyper::header::HeaderValue::from_static("br"),
            );
            response.body(Bytes::from(compressed)).unwrap()
        }
    }
}

fn compress_bytes(mut bytes: &[u8], brotli_params: &BrotliEncoderParams) -> Vec<u8> {
    let mut compressed = Vec::with_capacity(bytes.len());
    BrotliCompress(&mut bytes, &mut compressed, brotli_params).unwrap();
    compressed
}

fn to_triage_response(result: ServerResult<api::triage::Response>) -> Response {
    match result {
        Ok(result) => {
            let response = http::Response::builder().header_typed(ContentType::text());
            response.body(Bytes::from(result.0)).unwrap()
        }
        Err(err) => http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header_typed(ContentType::text_utf8())
            .body(Bytes::from(err))
            .unwrap(),
    }
}

async fn run_server(
    ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let server = Server::new(ctxt);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("accept error: {e:?}");
                continue;
            }
        };
        let server = server.clone();
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(move |req: Request| {
                let ctx = server.clone();
                async move {
                    let start = std::time::Instant::now();
                    let desc = format!("{} {}", req.method(), req.uri());
                    let mut r = serve_req(ctx, req).await;
                    let dur = start.elapsed();
                    info!("{}: {:?} {:?}", desc, r.as_ref().map(|r| r.status()), dur);
                    if let Ok(r) = &mut r {
                        r.headers_mut().insert(
                            hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                            hyper::header::HeaderValue::from_static("*"),
                        );
                    }
                    r.map(|r| r.map(Full::new))
                }
            });
            if let Err(e) =
                hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(TokioIo::new(stream), svc)
                    .await
            {
                eprintln!("connection error: {e:?}");
            }
        });
    }
}

pub async fn start(ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, port: u16) -> anyhow::Result<()> {
    let mut server_address: SocketAddr = "0.0.0.0:2346".parse().unwrap();
    server_address.set_port(port);
    run_server(ctxt, server_address).await
}

pub trait ResponseHeaders {
    fn header_typed<T: headers::Header>(self, h: T) -> Self;
}

impl ResponseHeaders for http::response::Builder {
    fn header_typed<T: headers::Header>(mut self, h: T) -> Self {
        let mut v = vec![];
        h.encode(&mut v);
        for value in v {
            self = self.header(T::name(), value);
        }
        self
    }
}
