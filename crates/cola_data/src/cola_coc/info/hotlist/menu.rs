// data/src/cola_coc/info/hotlist/menu.rs -- 数据 - COC - info - 上热门 - 菜单信息
// 2026/9/1 05:18 Created.

////////

use crate::cola_video::entity::comment::VideoCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 运营中心 - 上热门菜单信息
/// * `desc`: `安全的视频评论信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotlistMenuInfo {
    pub id: i64,                           // 评论 ID
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub parent_id: Option<i64>,            // 父评论 ID（可选）
    pub comment_type: i16,                 // 类型 (1. 文字 2. 语音 3. 照片 4. 视频等)
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub media_ids: Option<Vec<i64>>,       // 媒体对象
    pub likes: i32,                        // 点赞数量
    pub dislikes: i32,                     // 不喜欢数量
    pub collects: i32,                     // 收藏量
    pub reply: i32,                        // 回复数量
    pub is_pinned: Option<bool>,           // 是否置顶
    pub visibility: i16,                   // 可见范围
    pub add_time: i64,                     // 发布时间 (兼容旧版PHP)
    pub created_at: Option<DateTime<Utc>>, // 创建时间 (新版)
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

// 构造实现
impl HotlistMenuInfo {
    //

    ////////

    /// 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“评论不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            uid: 0,
            video_id: 0,
            parent_id: None,
            comment_type: 1,
            content: "上热门菜单不存在或已被删除".to_string(),
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
            is_pinned: Some(false),
            add_time: 0,
            created_at: None,
            updated_at: None,
        }
    }

    /// 2. [FROM] - 转换
    /// * `desc`: `纯净的从数据库实体转换为评论域模型`
    pub fn from_entity(entity: VideoCommentEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            uid: entity.uid,
            video_id: entity.video_id,
            parent_id: entity.parent_id,
            comment_type: entity.comment_type,
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
            is_pinned: entity.is_pinned,
            add_time: entity.add_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//////// END
