// src/network/http.rs -- 网络层 - HTTP 请求发送 (基于接入点)
// 2026/10/16 07:00 Created.

////////

use std::time::Duration;

use serde::Serialize;
use serde_json::Value;

use crate::error::{Result, SdkError};
use crate::network::endpoint::EndpointAddress;

////////

/// # [NETWORK] - HTTP 请求方法
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

/// # [NETWORK] - HTTP 路由描述 (由 handler 层定义)
/// * `desc`: `只描述路径与方法, 不包含任何业务逻辑`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HttpRoute {
    pub method: HttpMethod, // 请求方法
    pub path: &'static str, // 请求路径, 以 / 开头
}

/// # [NETWORK] - HTTP 发送器
/// * `desc`: `基于接入点地址同步发送请求并返回 JSON 响应`
pub struct HttpSender {
    timeout: Duration, // 单次请求超时
}

impl HttpSender {
    /// # 1. [NETWORK] - 创建发送器
    /// * `desc`: `创建带超时配置的同步 HTTP 发送器`
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    /// # 2. [NETWORK] - 发送 JSON 请求
    /// * `desc`: `向接入点发送请求并把响应体解析为 JSON`
    /// * `condition`: `非 2xx 状态码或响应非 JSON 时返回错误`
    pub fn send_json<B: Serialize>(
        &self,
        address: &EndpointAddress, // 目标接入点地址
        route: HttpRoute,          // 请求路由 (方法 + 路径)
        body: Option<&B>,          // 请求体, GET 可为空
    ) -> Result<Value> {
        let url = build_url(address, route.path);
        tracing::info!("[🗣️ NETWORK] - 发送请求: {:?} {}", route.method, url);

        let agent = ureq::AgentBuilder::new().timeout(self.timeout).build();
        let request = match route.method {
            HttpMethod::Get => agent.get(&url),
            HttpMethod::Post => agent.post(&url),
        };

        let outcome = match body {
            Some(payload) => {
                let value = serde_json::to_value(payload)
                    .map_err(|e| SdkError::InvalidBatch(format!("encode body failed: {e}")))?;
                request.send_json(value)
            }
            None => request.call(),
        };

        match outcome {
            Ok(response) => response
                .into_json::<Value>()
                .map_err(|e| SdkError::Storage(format!("invalid json response: {e}"))),
            Err(ureq::Error::Status(code, _)) => {
                tracing::warn!("[🤐 NETWORK] - ❌️ 请求失败, status={}", code);
                Err(SdkError::Storage(format!("http status {code}")))
            }
            Err(e) => {
                tracing::warn!("[🤐 NETWORK] - ❌️ 请求异常: {}", e);
                Err(SdkError::Storage(format!("http transport error: {e}")))
            }
        }
    }
}

////////

/// # [NETWORK] - 拼接请求 URL
/// * `desc`: `根据接入点地址与路径生成完整 URL, 默认 http 协议`
fn build_url(address: &EndpointAddress, path: &str) -> String {
    let host = &address.connect_host;
    let base = if host.starts_with("http://") || host.starts_with("https://") {
        host.trim_end_matches('/').to_string()
    } else {
        format!("http://{}:{}", host, address.port)
    };
    format!("{}{}", base, path)
}

//////// END
