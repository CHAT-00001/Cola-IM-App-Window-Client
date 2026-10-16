// cola_data/src/code/entity/risk/risk_subject.rs
// 数据 - AUTH - entity - 风险 - 风险实体表
// 2026/7/8 09:44

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 认证中心 - 风险实体
/// * `pg schema`: `cola_auth` - PG 模式
/// * `table name`: `rick_subject`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct RiskSubjectEntity {
    pub id: i64,                                  // 风险实体 ID
    pub _id: Option<String>,                      // UUID v4
    pub subject_type: i16,                        // 实体类型(唯一索引)
    pub subject_key: String,                      // 记录值(唯一索引)
    pub risk_reason: i16,                         // 风险原因
    pub risk_level: i16,                          // 风险等级
    pub is_banned: bool,                          // 是否封禁
    pub is_active: bool,                          // 是否活跃
    pub remark: Option<String>,                   // 备注
    pub request_total_count: i64,                 // 总请求数量
    pub status: i16,                              // 状态码
    pub banned_begin_at: Option<DateTime<Utc>>,   // 封禁开始时间
    pub banned_expired_at: Option<DateTime<Utc>>, // 封禁截止时间
    pub last_active_at: i64,                      // 最近一次活跃/心跳时间戳
    pub is_deleted: bool,                         // 逻辑删除
    pub created_at: DateTime<Utc>,                // 创建时间
    pub updated_at: DateTime<Utc>,                // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,        // 删除时间
}

////////

/// # 1. 统一的风险实体查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const RISK_SUBJECT_COLUMNS: &str = r#"
    id, _id, subject_type, subject_key, risk_reason, risk_level,
    is_banned, is_active, remark, request_total_count, status,
    banned_begin_at, banned_expired_at, last_active_at,
    is_deleted, created_at, updated_at, deleted_at
"#;

//////// END
