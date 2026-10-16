// video/src/gate_http.rs  -- 统一的请求查询体
// 2026/4/12 12:55 by wx: cestbon10080

////////

use crate::app::data::AppData;
use crate::app::error;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

////////

/// # ApiQuery
/// 统一的 API 请求查询
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiQuery {
    pub uid: Option<i64>,              // 当前操作用户ID
    pub device_id: Option<String>,     // 设备唯一ID
    pub log_id: Option<String>,        // 日志 Trace ID（链路追踪）
    pub access_token: Option<String>,  // 仅用于认证阶段
    pub refresh_token: Option<String>, // 仅用于刷新阶段
    pub ip: Option<IpAddr>,            // 客户端IP（自动兼容 v4/v6）
    pub location: Option<Location>,    // 地理位置
    pub target_id: Option<i64>,        // 目标ID
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    pub lng: f64,
    pub lat: f64,
}

impl ApiQuery {
    /// 空上下文（内部调用）
    pub fn empty() -> Self {
        Self::default()
    }

    /// 已登录上下文
    pub fn with_uid(uid: i64) -> Self {
        Self {
            uid: Some(uid),
            ..Default::default()
        }
    }

    // ------------------------------
    // 链式构造器
    // ------------------------------

    pub fn with_access_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub fn with_refresh_token(mut self, token: String) -> Self {
        self.refresh_token = Some(token);
        self
    }

    // ------------------------------
    // 工具方法 (解决 state 报错的关键)
    // ------------------------------

    /// ✅ 统一获取 Token 的方法
    /// 解决：no method named `token` found for reference `&ApiQuery`
    pub fn token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// 是否登录
    pub fn is_login(&self) -> bool {
        self.uid.is_some()
    }

    /// 获取UID（安全）
    pub fn uid(&self) -> Option<i64> {
        self.uid
    }

    /// 必须登录，否则返回错误（统一 AppData 体系）
    pub fn require_uid(&self) -> ApiResult<i64> {
        self.uid
            .ok_or_else(|| AppData::err(error::NOT_LOGIN, "未登录", None))
    }

    /// 获取IP字符串（方便日志）
    pub fn ip_str(&self) -> String {
        self.ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "-".into())
    }
}

/// 统一返回结果
pub type ApiResult<T> = Result<T, AppData<()>>;
