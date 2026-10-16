// cola_data/src/code/entity/file.rs
// 数据 - 可乐用户 - entity - 身份表
// 2026/8/14 06:48 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 验证中心 - 身份表
/// * `pg schema`: `cola_auth` -- PG 模式
/// * `table name`: `identity`  -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct IdentityEntity {
    pub id: i64,                            // ID (自增 / 雪花)
    pub _id: Option<String>,                // UUID v4
    pub id_type: Option<i16>,               // 身份类型
    pub user_id: i64,                       // 用户 ID
    pub identifier: Option<String>,         // 身份识别码
    pub secret: Option<String>,             // 密钥摘要
    pub verified_at: Option<DateTime<Utc>>, // 验证时间
    pub status: i16,                        // 状态码
    pub is_deleted: Option<bool>,           // 逻辑删除
    pub create_time: i64,                   // 创建时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,  // 删除时间
}

////////

/// # 2.[COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const IDENTITY_COLUMNS: &str = r#"
    id, _id, id_type, user_id, identifier, secret,
    verified_at, status, is_deleted,
    create_time, created_at, updated_at, deleted_at
"#;

//////// END
