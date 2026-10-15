//! The HTTP client GPUI uses to load remote images.
//!
//! `img("https://…")` is not fetched by the element itself: GPUI's asset system
//! asks `App::http_client()` for the bytes, and the client shipped with GPUI is
//! a null implementation whose only answer is *"No HttpClient available"*. A
//! real client therefore has to be installed during startup — see `main()`:
//!
//! ```ignore
//! cx.set_http_client(std::sync::Arc::new(net::HttpClient::new()));
//! ```
//!
//! Each request runs on its own short-lived worker thread with its own
//! single-threaded tokio runtime, so neither the render loop nor GPUI's
//! background executor (already busy driving the image loaders) can be starved.
//! The worker hands its result back through a `oneshot` channel.

use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::time::Duration;

use gpui_kit::http_client::http::{HeaderValue, StatusCode};
use gpui_kit::http_client::{
    AsyncBody, HttpClient as HttpClientTrait, Inner, Request, Response, Result, Url, anyhow,
};

/// Same shape as `futures::future::BoxFuture<'static, T>`, which the
/// `HttpClient` trait returns.
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

const USER_AGENT: &str = concat!("client/", env!("CARGO_PKG_VERSION"));
const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);

/// [`HttpClient`] backed by reqwest.
///
/// [`HttpClient`]: HttpClientTrait
pub struct HttpClient {
    user_agent: HeaderValue,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            user_agent: HeaderValue::from_static(USER_AGENT),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClientTrait for HttpClient {
    fn user_agent(&self) -> Option<&HeaderValue> {
        Some(&self.user_agent)
    }

    /// `None`: reqwest already reads `HTTP_PROXY` / `HTTPS_PROXY` / the Windows
    /// proxy settings on its own.
    fn proxy(&self) -> Option<&Url> {
        None
    }

    fn send(&self, request: Request<AsyncBody>) -> BoxFuture<Result<Response<AsyncBody>>> {
        let (parts, body) = request.into_parts();
        let method = parts.method.as_str().to_string();
        let uri = parts.uri.to_string();
        // Header values are copied as strings so the worker thread owns plain
        // data and both sides stay independent of the caller's header map.
        let headers = parts
            .headers
            .iter()
            .filter_map(|(name, value)| {
                Some((
                    name.as_str().to_string(),
                    value.to_str().ok()?.to_string(),
                ))
            })
            .collect::<Vec<_>>();
        let body = buffered_body(body);

        let (tx, rx) = futures::channel::oneshot::channel();
        thread::Builder::new()
            .name("cola-net".to_string())
            .spawn(move || {
                let _ = tx.send(fetch(&method, &uri, &headers, body));
            })
            .expect("failed to spawn the HTTP worker thread");

        Box::pin(async move {
            rx.await
                .unwrap_or_else(|_| Err(anyhow!("the HTTP worker thread stopped")))
        })
    }
}

/// Runs one request to completion on the calling (worker) thread.
fn fetch(
    method: &str,
    uri: &str,
    headers: &[(String, String)],
    body: Vec<u8>,
) -> Result<Response<AsyncBody>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| anyhow!("failed to start the network runtime: {error}"))?;
    // reqwest 的客户端必须建在 Tokio 运行时上下文里，否则会 panic
    // （"there is no reactor running"），所以先 enter() 再构造。
    let _guard = runtime.enter();

    // A client per request: the connection pool lives inside the runtime that
    // created it, so neither is ever reused after the thread shuts down.
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|error| anyhow!("failed to build the HTTP client: {error}"))?;

    let mut builder = client.request(reqwest::Method::from_bytes(method.as_bytes())?, uri);
    for (name, value) in headers {
        builder = builder.header(name.as_str(), value.as_str());
    }
    if !body.is_empty() {
        builder = builder.body(body);
    }

    let response = runtime.block_on(client.execute(builder.build()?))?;
    let status = StatusCode::from_u16(response.status().as_u16())?;
    let mut builder = Response::builder().status(status);
    for (name, value) in response.headers() {
        if let Ok(value) = value.to_str() {
            builder = builder.header(name.as_str(), value);
        }
    }
    let body = runtime.block_on(response.bytes())?;
    Ok(builder.body(AsyncBody::from_bytes(body))?)
}

/// The bodies GPUI sends are empty (`img()` only issues GETs). In-memory bodies
/// are forwarded as they are and streaming ones are treated as empty.
fn buffered_body(body: AsyncBody) -> Vec<u8> {
    match body.0 {
        Inner::Empty => Vec::new(),
        Inner::Bytes(cursor) => cursor.into_inner().to_vec(),
        Inner::AsyncReader(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{AsyncBody, HttpClient, HttpClientTrait, buffered_body};

    #[test]
    fn empty_bodies_stay_empty() {
        assert!(buffered_body(AsyncBody::empty()).is_empty());
    }

    #[test]
    fn in_memory_bodies_are_forwarded() {
        assert_eq!(buffered_body(AsyncBody::from(vec![1, 2, 3])), vec![1, 2, 3]);
    }

    #[test]
    fn requests_without_a_body_are_not_used_as_payloads() {
        let body = AsyncBody::from_reader(futures::io::empty());
        assert!(buffered_body(body).is_empty());
    }

    /// 真实网络请求（默认被忽略）：
    ///
    /// ```text
    /// cargo test --release -- --ignored
    /// ```
    ///
    /// 用来确认「Windows 上不依赖任何 C 工具链的 TLS（SChannel）+ reqwest」这条链路
    /// 真的能下载到界面里那些 CDN 头像。
    #[test]
    #[ignore = "hits the network"]
    fn downloads_a_remote_avatar() {
        let client = HttpClient::new();
        let url = "https://cdn1.damawei.com/android_1000000206_20250716_082707_7338796.jpg?imageView2/1/w/100/h/100";
        let response = gpui_kit::block_on(client.get(url, AsyncBody::empty(), true))
            .expect("the request should succeed");
        assert!(
            response.status().is_success(),
            "unexpected status {}",
            response.status()
        );
    }
}
