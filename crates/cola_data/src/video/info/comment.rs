// data/src/video/info/comment.rs -- 数据 - VIDEO - info - 评论记录信息
// 2026/5/21 00:58 Created.

////////

use crate::video::entity::comment::VideoCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 评论记录信息
/// * `desc`: `安全的信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentInfo {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub message_type: i16,                 // 类型
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub media_ids: Option<Vec<i64>>,       // 媒体对象 (新版本)
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 不喜欢数量
    pub collects: i32,                     // 收藏量
    pub reply: i32,                        // 回复数量（是父评论时）
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub is_pinned: Option<bool>,           // 是否置顶
    pub is_deleted: Option<bool>,          // 是否删除
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

// 构造实现
impl CommentInfo {
    //

    ////////

    /// #1. [BUILD] - 空
    /// * `desc`: `专门用于返回“评论不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            user_id: 0,
            video_id: 0,
            parent_id: None,
            message_type: 1,
            content: "评论不存在或已被删除".to_string(),
            photos_url: None,
            video_url: None,
            voice_url: None,
            duration: None,
            media_ids: None,
            likes: 0,
            dislikes: 0,
            collects: 0,
            reply: 0,
            visibility: 1,
            region_code: None,
            status: 0,
            is_pinned: Some(false),
            is_deleted: Some(true),
            add_time: 0,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }

    ////////

    /// # 2. [FROM] - 转换
    /// `desc`: `纯净的从数据库实体转换为评论域模型`
    pub fn from_entity(entity: VideoCommentEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            parent_id: entity.parent_id,
            message_type: entity.message_type,
            content: entity.content,
            photos_url: entity.photos_url,
            video_url: entity.video_url,
            voice_url: entity.voice_url,
            duration: entity.duration,
            media_ids: entity.media_ids,
            likes: entity.likes,
            dislikes: entity.dislikes,
            collects: entity.collects,
            reply: entity.reply,
            visibility: entity.visibility,
            region_code: entity.region_code,
            status: entity.status,
            is_pinned: entity.is_pinned,
            is_deleted: entity.is_deleted,
            add_time: entity.add_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
