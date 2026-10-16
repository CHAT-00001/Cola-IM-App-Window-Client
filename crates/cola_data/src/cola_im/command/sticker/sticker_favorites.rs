// cola_data/src/im/entity/score/sticker.rs -- DATA - IM - entity - 资源 - 贴图
// 2026/3/30 05:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [COMMAND] - IM - 表情包贴图收藏命令
/// * `desc`: `用户侧 - 添加收藏表情包到自己名下`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct StickerAddFavoritesCmd {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4 (幂等)
    pub user_id: i64,                      // 用户 ID
    pub sticker_id: i64,                   // 贴图 ID
    pub packing_id: Option<i64>,           // 表情包 ID
    pub created_at: Option<DateTime<Utc>>, // 添加时间
}

////////

/// # [COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const STICKER_FAVORITES_COLUMNS: &str = r#"
    id, _id, vx_id, user_id, name, signature, sticker_url, media_id, label, language,
    perm_id, visibility_, views, likes, favorites, use_count, score, coin, status, is_deleted,
    created_at, updated_at, deleted_at
"#;

//////// END
