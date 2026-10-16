// cola_data/src/new/rntity/interact.rs  -- VIDEO - Entity - 互动
// 2026-01-16 09:37:10

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 浏览记录表
/// * table name: video_visited
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VisitedEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub video_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # [ENTITY] - 点赞表
/// * table name: video_like
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LikeEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub video_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # [ENTITY] - 不喜欢表
/// * table name: video_dislike
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DislikeEntity {
    pub id: i64,                           // id
    pub uid: Option<String>,               // 作者id
    pub video_id: Option<i64>,             // 视频 id
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub addtime: Option<i32>,              // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}
