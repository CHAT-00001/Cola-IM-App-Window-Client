// cola_data/src/cola_im/entity/gift/gift.rs
// 🗄 数据 - ⏹ 可乐IM - entityy - gift - 礼物表
// 2026/6/19 17:51

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 礼物 表
/// * `pg schema`: `cola_im`
/// * `table name`: `gift`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImGiftEntity {
    pub id: i64,                   // ID
    pub user_id: i64,              // 用户 ID
    pub classify_id: i64,          // 分类 ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub icon: String,              // 图标
    pub sort: i16,                 // 排序(默认9999)
    pub status: i16,               // 状态
    pub add_time: i64,             // 添加时间 - 机器
    pub upd_time: i64,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

////////

/// # [COLUMNS] - 统一的sqlx字段映射
pub const IM_GIFT_COLUMNS: &str = r#"
    id, user_id, classify_id, name, name_en, is_banned, is_active,
    url, sort, status,
    add_time, upd_time
"#;

//////// END
