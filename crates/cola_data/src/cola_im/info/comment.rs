// cola_data/src/new/info/add  -- 数据中心 - VIDEO - 信息 - 评论
// 2026/5/21 00:58

////////

use serde::{Deserialize, Serialize};
use crate::video::entity::comment::VideoCommentEntity;

////////

/// # [INFO] - 视频 评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentInfo {
    pub id: i64,                // 评论 ID
    pub user_id: i64,           // 用户 ID
    pub video_id: i64,          // 视频 ID
    pub parent_id: Option<i64>, // 爸爸评论（可选）
    pub content: String,        // 内容
    pub likes: i32,             // 点赞数量
    pub reply: i32,             // 回复数量
    pub send_time: i64,         // 发送时间
    pub sync_time: i64,         // 同步时间
}

/// # 构造评论
impl CommentInfo {
    /// 构造评论基础信息，并自动判定是否为视频作者
    pub fn new(
        id: i64,
        user_id: i64,
        video_id: i64,
        parent_id: Option<i64>,
        content: String,
        likes: i32,
        reply: i32,
        send_time: i64,
        sync_time: i64,
        video_author_id: i64, // 传入视频作者的 UID
    ) -> Self {
        // 如果评论的发布者 ID 等于 视频作者的 ID，则为 true
        let is_author = user_id == video_author_id;

        Self {
            id,
            user_id,
            video_id,
            parent_id,
            content,
            likes,
            reply,
            send_time,
            sync_time,
        }
    }

    /// 纯净的从数据库实体转换为评论域模型
    pub fn from_entity(entity: VideoCommentEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            parent_id: entity.parent_id,
            content: entity.content,
            likes: entity.likes,
            reply: entity.reply,
            send_time: entity.send_time,
            sync_time: entity.sync_time,
        }
    }
}

//////// END
