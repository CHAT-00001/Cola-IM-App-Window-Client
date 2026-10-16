// cola_data/src/code/entity/service.rs  -- 数据 - 认证中心 - handler - service
// 2026/5/23 07:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 认证中心 - 会话
/// * `pg schema`: `code` - PG 模式
/// * `table name`: `service` - 表名
/// * platform 字段在 DB 中是 INT NOT NULL，用 i16 对齐
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AuthSessionEntity {
    pub id: i64,                                   // 会话 ID (PG 大自增)
    pub user_id: i64,                              // 用户 ID
    pub access_token: String,                      // 访问临牌 Token
    pub refresh_token: String,                     // 刷新 Token 哈希值
    pub client_id: String,                         // 客户端 ID
    pub device_id: String,                         // 设备号 ID
    pub access_expires_at: i64,                    // 临时token过期时间
    pub refresh_expires_at: i64,                   // 刷新token过期时间
    pub last_active_at: i64,                       // 最后活跃时间戳
    pub status: i16,                               // 状态: 1有效, 0注销, -1被挤掉
    pub created_at: chrono::DateTime<chrono::Utc>, // 创建时间
    pub updated_at: chrono::DateTime<chrono::Utc>, // 同步更新时间
    pub platform: i16, // 平台 INT -> i16: 0=未知, 1=ios, 2=android, 3=web
}

////////

/// #[COLUMNS] -  统一的登录会话查询字段（完全对应你最新的 AuthSessionEntity 字段）
pub const AUTH_SESSION_COLUMNS: &str = r#"
    id, user_id, access_token, refresh_token, client_id,
    device_id, access_expires_at,refresh_expires_at, last_active_at,
    status, created_at, updated_at, platform
"#;

////////
