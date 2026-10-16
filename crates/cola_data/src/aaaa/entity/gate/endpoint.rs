// cola_data/src/aaaa/entity/handler/endpoint.rs  -- DATA - AAAA - entity - handler - 全球网关接入点
// 2026/8/14 13:00

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 全球网关 - 接入点(Endpoint / Backend)
/// * `desc`: `一个 endpoint 即一个 backend, 拥有独立域名与证书`
/// * `pg schema`: `cola_aaaa` -- PG 模式
/// * `table name`: `endpoint` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GatewayEndpointEntity {
    pub id: i64,                                  // ID (自增 / 雪花)
    pub _id: Option<String>,                      // UUID v4
    pub app_id: Option<String>,                   // 所属应用/模块标识
    pub biz_id: String,                           // 业务域标识: gateway / api / ws / wt ...
    pub endpoint_code: String,                    // 接入点编码 (全局唯一)
    pub endpoint_name: String,                    // 接入点名称
    pub endpoint_domain: String,                  // 接入点域名 (如: https://api-sgr-01.example.com)
    pub region_code: String,                      // 归属区域编码 (如: sgr / hk / us)
    pub region_name: String,                      // 归属区域名称
    pub is_https: bool,                           // 是否启用 HTTPS
    pub cert_expire_at: Option<DateTime<Utc>>,    // 证书到期时间 (运维监控用)
    pub auth_type: i16, // 鉴权类型: 0-无鉴权(公开), 1-URL鉴权A, 2-URL鉴权B
    pub auth_key: Option<String>, // 边缘鉴权密钥 (防盗链签名 Key)
    pub health_status: i16, // 节点健康: 0-未知, 1-健康, 2-异常
    pub duty_status: i16, // 执勤状态: 0-停用, 1-执勤, 2-维护 (运维手动置位)
    pub last_heartbeat_at: Option<DateTime<Utc>>, // 最近一次探活时间
    pub is_enabled: bool, // 是否启用
    pub status: i16,    // 状态码: 1-正常, 0-失效
    pub is_deleted: bool, // 逻辑删除
    pub created_time: i64, // 创建时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

/// # 2.[COLUMNS] - 数据表原始字段
pub const GATEWAY_ENDPOINT_COLUMNS: &str = r#"
    id, _id, app_id, biz_id, endpoint_code, endpoint_name, endpoint_domain,
    region_code, region_name, is_https, cert_expire_at, auth_type, auth_key,
    health_status, duty_status, last_heartbeat_at, is_enabled, status,
    is_deleted, created_time, created_at, updated_at, deleted_at
"#;

//////// END
