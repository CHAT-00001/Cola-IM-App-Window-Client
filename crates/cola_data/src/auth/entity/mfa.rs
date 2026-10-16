// cola_data/src/code/entity/mfa.rs  -- 数据中心 - AUTH - entity - MFA
// 2026/7/8 09:37

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 认证中心 - 用户绑定的地区表
/// * `table name`: `auth_mfa`
/// * `status`: 状态: 1-正常, 0-已注销, -1-被挤下线, -2-已被管理员强踢/禁用
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AuthMfaEntity {
    pub id: i64,                          // 主键 ID (PG 大自增)
    pub user_id: i64,                     // 用户 ID
    pub _sn: String,                      // 序列号 (UUID)
    pub code: String,                     // 名称
    pub email: String,                    // 中文名称
    pub is_banned: bool,                  // 是否封禁
    pub is_active: bool,                  // 是否活跃
    pub access_key: String,               // 公钥
    pub secret_key: String,               // 秘钥
    pub status: i16,                      // 状态
    pub banned_expired_at: i64,           // 封禁时间截止 - 机器
    pub last_active_at: i64,              // 最近一次活跃/心跳时间戳
    pub created_at: DateTime<Utc>,        // 创建时间
    pub updated_at: DateTime<Utc>,        // 更新时间
    pub banned_at: Option<DateTime<Utc>>, // 封禁时间
}

////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const IAM_MFA_COLUMNS: &str = r#"
    id, user_id, _sn, app, email, is_banned, is_active,
    access_key, secret_key, status, is_online, status,
    banned_expired_at, last_active_at, created_at, updated_at, banned_at
"#;

//////// END
