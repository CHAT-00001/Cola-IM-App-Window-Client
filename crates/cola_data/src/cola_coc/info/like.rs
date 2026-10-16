// data/src/video/info/like.rs
// 🗄 数据 - ▶ 视频 - info - 点赞信息
// 2026/8/9 21:58 Created.

////////

use crate::video::entity::like::like::VideoLikeEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 点赞信息
/// * `desc`: `安全的信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoLikeInfo {
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

// 构造实现
impl VideoLikeInfo {
    /// 1. 专门用于返回“点赞不存在”的空对象
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            uid: 0,
            video_id: 0,
            status: 0,
            addtime: 0,
            is_deleted: Some(true),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }

    /// 2. 纯净的从数据库实体转换为点赞域模型
    pub fn from_entity(entity: VideoLikeEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            uid: entity.uid,
            video_id: entity.video_id,
            status: entity.status,
            addtime: entity.addtime,
            is_deleted: entity.is_deleted,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
