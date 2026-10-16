// cola_data/src/gift/entity/expression.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 表情
// 2026/6/19 17:58

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 表情表
/// * `pg schema`: `cola_im` - PG 模式
/// * `table name`: `expression` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImExpressionEntity {
    pub id: i64,                   // ID
    pub user_id: i64,              // 用户 ID
    pub classify_id: i64,          // 分类 ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub tag: Option<String>,       // 标签
    pub url: String,               // 地址
    pub sort: i16,                 // 排序
    pub status: i16,               // 状态
    pub is_banned: Option<bool>,   // 是否禁用
    pub is_newest: Option<bool>,   // 是否最新
    pub add_time: i64,             // 添加时间 (兼容旧版)
    pub upd_time: i64,             // 更新时间 (兼容旧版)
    pub created_at: DateTime<Utc>, // 创建时间
    pub updated_at: DateTime<Utc>, // 更新时间
}

////////

/// # 1. [COLUMNS] - 统一的SQLx查询字段
pub const IM_EXPRESSION_COLUMNS: &str = r#"
    id, user_id, classify_id, name, name_en, tag,
    url, sort, status, is_banned, is_newest,
    add_time, upd_time
"#;

//////// END
