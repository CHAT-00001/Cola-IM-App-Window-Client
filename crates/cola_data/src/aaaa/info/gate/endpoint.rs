// data/src/aaaa/info/handler/endpoint.rs -- DATA - AAAA - 网关 - 接入点信息
// 2026/8/9 07:38

////////

use crate::aaaa::entity::gate::endpoint::GatewayEndpointEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 全球网关 - 接入点安全脱敏视图(隐藏鉴权密钥)
/// * `desc`: `客户端可见的接入点信息, 不返回 entity 原始结构`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub id: String,                              // 接入点 ID
    pub app_id: Option<String>,                  // 所属应用
    pub biz_id: String,                          // 业务域: gateway / api / ws / wt
    pub endpoint_code: String,                   // 接入点编码
    pub endpoint_name: String,                   // 接入点名称
    pub endpoint_domain: String,                 // 接入点域名
    pub region_code: String,                     // 归属区域编码
    pub region_name: String,                     // 归属区域名称
    pub is_https: bool,                          // 是否启用 HTTPS
    pub cert_expire_at: Option<DateTime<Utc>>,   // 证书到期时间
    pub auth_type: i16,                          // 鉴权类型
    pub health_status: i16,                      // 节点健康: 0-未知, 1-健康, 2-异常
    pub duty_status: i16,                        // 执勤状态: 0-停用, 1-执勤, 2-维护
    pub last_heartbeat_at: Option<DateTime<Utc>>,// 最近探活时间
    pub is_enabled: bool,                        // 是否启用
    pub status: i16,                             // 状态码
    pub created_at: DateTime<Utc>,               // 创建时间
}

// 构造实现 (entity -> info, 剔除 auth_key 等敏感字段)
impl From<GatewayEndpointEntity> for EndpointInfo {
    fn from(e: GatewayEndpointEntity) -> Self {
        Self {
            id: e.id.to_string(),
            app_id: e.app_id,
            biz_id: e.biz_id,
            endpoint_code: e.endpoint_code,
            endpoint_name: e.endpoint_name,
            endpoint_domain: e.endpoint_domain,
            region_code: e.region_code,
            region_name: e.region_name,
            is_https: e.is_https,
            cert_expire_at: e.cert_expire_at,
            auth_type: e.auth_type,
            health_status: e.health_status,
            duty_status: e.duty_status,
            last_heartbeat_at: e.last_heartbeat_at,
            is_enabled: e.is_enabled,
            status: e.status,
            created_at: e.created_at.unwrap_or_else(Utc::now),
        }
    }
}

//////// END
