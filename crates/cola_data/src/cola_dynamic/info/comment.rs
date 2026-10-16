// cola_data/src/cola_dynamic/info/file.rs
// 🗄 数据 - ⏹ 可乐动态 - info - 评论信息
// 2026/5/21 00:58

////////

use crate::cola_dynamic::entity::comment::DynamicCommentEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 动态 评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicCommentInfo {
    pub id: i64,                // 评论 ID
    pub user_id: i64,           // 用户 ID
    pub dynamic_id: i64,        // 动态 ID
    pub parent_id: Option<i64>, // 父级评论（可选）
    pub content: String,        // 内容
    pub likes: i32,             // 点赞数量
    pub reply: i32,             // 回复数量
    pub add_time: i64,          // 发送时间
    pub upd_time: Option<i64>,  // 同步时间
}

// 构造评论
impl DynamicCommentInfo {
    //

    ////////

    /// # [BUILD] - 新建
    pub fn new(
        id: i64,                // 评论 ID
        user_id: i64,           // 用户 ID
        dynamic_id: i64,        // 动态 ID
        parent_id: Option<i64>, // 父级评论 ID
        content: String,        // 内容
        likes: i32,             // 点赞数量
        reply: i32,             // 不喜欢数量
        add_time: i64,          // 发送时间
        upd_time: i64,          // 同步时间
        video_author_id: i64,   // 传入视频作者的 UID
    ) -> Self {
        // 如果评论的发布者 ID 等于 视频作者的 ID，则为 true
        let _is_author = user_id == video_author_id;

        Self {
            id,
            user_id,
            dynamic_id,
            parent_id,
            content,
            likes,
            reply,
            add_time,
            upd_time: Some(upd_time),
        }
    }

    ////////

    /// # [FROM] - 转换
    pub fn from_entity(entity: DynamicCommentEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.uid,
            dynamic_id: entity.dynamic_id,
            parent_id: entity.parent_id,
            content: entity.content,
            likes: entity.likes,
            reply: entity.reply,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
        }
    }
}

//////// END
