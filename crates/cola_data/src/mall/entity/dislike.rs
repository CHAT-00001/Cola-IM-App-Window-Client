// cola_data/src/video/entity/dislike.rs -- 数据 - VIDEO - Entity - 不喜欢记录表
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
    pub id: i64,                           // ID (自增 / 雪花)
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub add_time: Option<i64>,             // 创建时间（兼容PHP旧版）
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

////////

/// # [CONSTANT] - 视频不喜欢表字段常量定义
pub const VIDEO_DISLIKE_COLUMNS: &str = "\
    id, user_id, video_id, status, is_deleted \
    add_time, created_at, updated_at\
    ";

//////// END
