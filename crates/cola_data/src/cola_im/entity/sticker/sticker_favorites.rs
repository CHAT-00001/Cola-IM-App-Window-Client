// cola_data/src/im/entity/sticker/sticker_favorites.rs -- DATA - IM - entity - 贴图 - 收藏
// 2026/3/30 05:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 表情包贴图收藏记录表
/// * `pg schema`: `cola.im` -- PG 模式
/// * `table name`: `sticker_favorites`  -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct StickerFavoritesEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4 (幂等)
    pub user_id: i64,                      // 用户 ID
    pub sticker_id: i64,                   // 贴图 ID
    pub packing_id: Option<i64>,           // 表情包 ID
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const STICKER_FAVORITES_COLUMNS: &str = r#"
    id, _id, user_id, sticker_id, packing_id, status, is_deleted,
    created_at, updated_at, deleted_at
"#;

//////// END
