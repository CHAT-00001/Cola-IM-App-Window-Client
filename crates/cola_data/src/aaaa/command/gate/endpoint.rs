// crates/cola_data/src/aaaa/command/handler/endpoint.rs -- DATA - AAAA - COMMAND - ENDPOINT
// 2026/10/15 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [CMD] - 创建全球网关接入点
/// * `desc`: `字段与 gateway.endpoint 表对齐`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointCreateCmd {
    pub _id: Option<String>,
    pub app_id: Option<String>,
    pub biz_id: String,
    pub endpoint_code: String,
    pub endpoint_name: String,
    pub endpoint_domain: String,
    pub region_code: String,
    pub region_name: String,
    pub is_https: bool,
    pub cert_expire_at: Option<DateTime<Utc>>,
    pub auth_type: i16,
    pub auth_key: Option<String>,
    pub health_status: i16,
    pub duty_status: i16,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub is_enabled: bool,
    pub status: i16,
    #[serde(default)]
    pub is_deleted: bool,
}

////////

/// # [CMD] - 更新全球网关接入点
/// * `desc`: `所有字段可选，None 表示保留数据库原值`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EndpointUpdateCmd {
    pub app_id: Option<String>,
    pub biz_id: Option<String>,
    pub endpoint_code: Option<String>,
    pub endpoint_name: Option<String>,
    pub endpoint_domain: Option<String>,
    pub region_code: Option<String>,
    pub region_name: Option<String>,
    pub is_https: Option<bool>,
    pub cert_expire_at: Option<DateTime<Utc>>,
    pub auth_type: Option<i16>,
    pub auth_key: Option<String>,
    pub health_status: Option<i16>,
    pub duty_status: Option<i16>,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub is_enabled: Option<bool>,
    pub status: Option<i16>,
    pub is_deleted: Option<bool>,
}

//////// END
