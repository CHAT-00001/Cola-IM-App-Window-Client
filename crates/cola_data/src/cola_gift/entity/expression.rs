// data/src/gift/entity/expression.rs
// 🗄 数据 - ⏹ 可乐资产 - entity - gift
// 2026/6/19 17:58

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 表情 表
/// * `pg schema`: `cola_fs`
/// * `table name`: `expression`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GiftExpressionEntity {
    pub id: i64,                   // ID
    pub user_id: i64,              // 用户 ID
    pub classify_id: i64,          // 分类 ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub url: String,               // 地址
    pub sort: i16,                 // 排序
    pub status: i16,               // 状态
    pub add_time: i64,             // 添加时间 - 机器
    pub upd_time: i64,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const GIFT_EXPRESSION_COLUMNS: &str = r#"
    id, user_id, classify_id, name, name_en, is_banned, is_active,
    url, sort, status,
    add_time, upd_time
"#;

//////// END
