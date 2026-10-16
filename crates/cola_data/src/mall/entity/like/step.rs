// cola_data/src/video/entity/interact/dislike
// 数据 - VIDEO - Entity - 互动 - 踩
// 2026/8/2 15:08 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 踩踏表
/// * `pg schema`: `video`
/// * `table name`: `video_step`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoStepEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub video_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # [CONSTANT] - 视频踩常量
pub const VIDEO_STEP_COLUMNS: &str = "\
    id, uid, video_id, status, \
    addtime, created_at, updated_at\
    ";

//////// END
