// cola_data/src/video/entity/interact/dislike.rs
// 数据 - VIDEO - Entity - 互动 - 不喜欢
// 2026/8/2 15:08 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 不喜欢表
/// * `pg schema`: `video`
/// * `table name`: `video_dislike`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoDislikeEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub video_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # [CONSTANT] - 视频不喜欢表字段常量定义
pub const VIDEO_DISLIKE_COLUMNS: &str = "\
    id, uid, video_id, status, \
    addtime, created_at, updated_at\
    ";

//////// END
