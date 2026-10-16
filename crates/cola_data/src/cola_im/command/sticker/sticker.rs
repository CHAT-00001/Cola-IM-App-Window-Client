// cola_data/src/im/cmd/sticker/sticker.rs -- DATA - IM - cmd - 表情包 - 贴图
// 2026/3/30 05:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [COMMAND] - IM - 表情包贴图创建命令
/// * `desc`: `用户侧 - 创建新表情包贴图`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct StickerCreateCmd {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub vx_id: Option<String>,             // UUID v7
    pub user_id: i64,                      // 用户 ID
    pub name: Option<String>,              // 名称
    pub signature: Option<String>,         // 签名
    pub sticker_url: Option<String>,       // 贴图URL
    pub media_id: Option<i64>,             // S3 媒体 ID
    pub label: Option<String>,             // 标签
    pub language: Option<String>,          // 语言
    pub perm_id: i16,                      // 权限
    pub visibility_: i16,                  // 可见范围
    pub views: i64,                        // 流量数量
    pub likes: i64,                        // 点赞数量
    pub favorites: i64,                    // 收藏数量
    pub use_count: i64,                    // 使用数量
    pub score: i64,                        // 来源
    pub coin: i64,                         // 钻石
    pub status: Option<i16>,               // 状态
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

// 构造实现

impl StickerCreateCmd {

}


//////// END
