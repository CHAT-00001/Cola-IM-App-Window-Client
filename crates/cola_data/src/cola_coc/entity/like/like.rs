// cola_data/src/video/entity/like.rs
// 数据 - ▶ 视频 - Entity - 点赞
// 2026/8/2 15:08 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 点赞表
/// * `pg schema`: `video` - PG 模式
/// * `table name`: `like` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoLikeEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: i64,                      // 创建时间（兼容旧版PHP）
    pub is_deleted: Option<bool>,          // 是否删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

/// # [CONSTANT] - 视频喜欢表字段常量定义
pub const VIDEO_LIKE_COLUMNS: &str = r#"
    id, _id, uid, video_id, status,
    addtime, is_deleted, created_at, updated_at, deleted_at
    "#;

//////// END+
