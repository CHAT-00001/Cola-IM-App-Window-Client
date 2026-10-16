// cola_infra/src/handler/auth/code.rs -- INFRA - HANDLER - AUTH - 验证码接口路径与方法
// 2026/4/12 14:45 Created.

////////

use serde::Serialize;
use serde_json::Value;

use crate::error::{Result, SdkError};
use crate::network::endpoint::{EndpointRegistry, TransportKind};
use crate::network::http::{HttpMethod, HttpRoute, HttpSender};
use crate::storage::sqlite::SqliteStore;
use crate::util::verify::verify_phone_no;

////////

/// # [HANDLER] - 发送验证码路由
/// * `desc`: `POST /api/v2/auth/captcha, 仅描述路径与方法`
pub const SEND_CAPTCHA_ROUTE: HttpRoute = HttpRoute {
    method: HttpMethod::Post,
    path: "/api/v2/auth/captcha",
};

/// # [HANDLER] - 验证码业务标识 (用于接入点选择)
pub const AUTH_BIZ_ID: &str = "auth";

////////

/// # [HANDLER] - 发送验证码请求体
#[derive(Debug, Clone, Serialize)]
pub struct CaptchaRequest {
    pub area: String,     // 区号 "+86"
    pub phone_no: String, // 手机号
}

/// # 1. [HANDLER] - 发送验证码
/// * `desc`: `校验手机号 -> 选择 auth 接入点 -> 发送请求 -> 原始 JSON 落库 -> 返回 JSON`
/// * `condition`: `手机号非法、无可用接入点、HTTP 失败或存储失败时返回错误`
pub fn send_captcha(
    registry: &EndpointRegistry, // 接入点注册表
    sender: &HttpSender,         // HTTP 发送器
    store: &SqliteStore,         // SQLite 仓储
    application_id: &str,        // 应用 ID (验证码按应用隔离)
    area: &str,                  // 区号
    phone_no: &str,              // 手机号
) -> Result<Value> {
    verify_phone_no(area, phone_no)?;

    let endpoint = registry
        .select(AUTH_BIZ_ID, TransportKind::Http)
        .map_err(|e| SdkError::NotFound(format!("{e:?}")))?;

    let body = CaptchaRequest {
        area: area.to_string(),
        phone_no: phone_no.to_string(),
    };
    let json = sender.send_json(&endpoint.address, SEND_CAPTCHA_ROUTE, Some(&body))?;

    store.insert_captcha(
        application_id,
        area,
        phone_no,
        200,
        &json.to_string(),
    )?;
    tracing::info!("[🗣️ AUTH HANDLER]: ✅️ 验证码已下发并落库: area={}", area);
    Ok(json)
}

////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::endpoint::{EndpointAddress, EndpointProtocol, EndpointStatus, GatewayEndpoint};
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    /// # [TEST] - 启动一次性本地 HTTP 服务, 返回指定 JSON
    /// * `desc`: `监听随机端口, 响应一次请求后退出, 返回监听端口`
    fn spawn_mock(body: &'static str) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        port
    }

    /// # [TEST] - 构造只包含 auth 接入点的注册表
    /// * `desc`: `把 auth 接入点指向本地 mock 端口`
    fn registry_for(port: u16) -> EndpointRegistry {
        let mut registry = EndpointRegistry::from_builtin();
        registry.replace_endpoints(vec![GatewayEndpoint {
            id: "auth-mock".to_string(),
            biz_id: AUTH_BIZ_ID.to_string(),
            priority: 100,
            status: EndpointStatus::Active,
            address: EndpointAddress {
                connect_host: "127.0.0.1".to_string(),
                port,
                tls_server_name: None,
                authority: None,
            },
            protocols: EndpointProtocol::http_websocket(),
            health: Default::default(),
        }]);
        registry
    }

    #[test]
    fn rejects_invalid_phone_without_network_call() {
        let registry = registry_for(1);
        let sender = HttpSender::new(Duration::from_secs(2));
        let store = SqliteStore::open_in_memory().unwrap();
        let result = send_captcha(&registry, &sender, &store, "cola_im", "+86", "12");
        assert!(result.is_err());
        assert!(store.latest_captcha_response("cola_im", "+86", "12").unwrap().is_none());
    }

    #[test]
    fn sends_captcha_and_persists_json_per_application() {
        let port = spawn_mock(r#"{"code":0,"message":"ok"}"#);
        let registry = registry_for(port);
        let sender = HttpSender::new(Duration::from_secs(2));
        let store = SqliteStore::open_in_memory().unwrap();

        let json = send_captcha(&registry, &sender, &store, "cola_im", "+86", "13800138000")
            .unwrap();
        assert_eq!(json["code"], 0);

        let stored = store
            .latest_captcha_response("cola_im", "+86", "13800138000")
            .unwrap()
            .unwrap();
        assert!(stored.contains("\"message\":\"ok\""));
        // 其他应用不可见
        assert!(store
            .latest_captcha_response("other_app", "+86", "13800138000")
            .unwrap()
            .is_none());
    }
}

//////// END
