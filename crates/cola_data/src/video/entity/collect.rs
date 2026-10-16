// cola_data/src/video/entity/collect.rs
// 🗄 数据 - ▶ 可乐视频 - entity - collect - 收藏
// 2026/5/20 19:05 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 收藏表
/// * `pg schema`: `video`
/// * `table name`: `collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoCollectEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub uid: i64,                          // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub folder_id: i64,                    // 收藏夹 ID：默认 0 (主收藏夹/未分类)
    pub channel_id: i16,                   // 通道 ID
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态：0失效 1有效
    pub add_time: i32,                     // 添加时间 (兼容旧版 PHP)
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub created_at: Option<DateTime<Utc>>, // 🕒 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 🕒 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 🕒 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 常量字段
pub const VIDEO_COLLECT_COLUMNS: &str = r#"
    id, uid, video_id, folder_id, channel_id,
    remark, status, add_time, is_deleted,
    created_at, updated_at, deleted_at,
"#;
//////// END
