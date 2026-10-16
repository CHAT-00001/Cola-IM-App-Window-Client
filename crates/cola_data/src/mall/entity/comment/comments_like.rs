// cola_data/src/video/entity/comment/comments_like.rs -- 数据 - VIDEO - entity - 评论 - 点赞表
// 2026/1/16 09:37 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 视频 - 评论点赞记录表
/// * `pg schema`: `video` - PG 模式
/// * `table name`: `comments_like` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoCommentLikeEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 用户 ID
    pub comment_id: i64,                   // 评论 ID
    pub video_id: Option<i64>,             // 视频 ID（可选）
    pub touid: Option<i64>,                // 目标用户 ID (可选)
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 是否删除
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_COMMENT_LIKE_COLUMNS: &str = r#"
    id, _id, user_id, comment_id, video_id, touid,
    status, is_deleted,
    add_time, created_at, updated_at, deleted_at
"#;

//////// END
