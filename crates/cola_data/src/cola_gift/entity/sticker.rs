// cola_data/src/gift/entity/score.rs  -- GIFT - entity - 自定义表情贴纸
// 2026/6/19 17:58

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const GIFT_STICKER_COLUMNS: &str = r#"
    id, user_id, classify_id, name, name_en, is_banned, is_active,
    url, sort, status,
    add_time, upd_time, created_at, updated_at,
"#;
////////

/// # [ENTITY] - 表情包贴纸 表
/// * `pg schema`: `cola_fs`
/// * `table name`: `score`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GiftStickerEntity {
    pub id: i64,                   // ID
    pub user_id: i64,              // 用户 ID
    pub classify_id: i64,          // 分类 ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub url: String,               // 地址
    pub sort: i16,                 // 排序
    pub status: i16,               // 状态
    pub addtime: i64,              // 添加时间 - 机器
    pub updtime: i64,              // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

//////// END
