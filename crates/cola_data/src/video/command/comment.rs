// cola_data/src/video/cmd/comment.rs -- 数据 - VIDEO - cmd - 评论 - mod
// 2026/5/20 12:01 Created.

////////

use crate::video::entity::comment::VideoCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum MessageType {
    Text = 1,
    Image = 2,
    Voice = 3,
    Video = 4,
    LivePhoto = 5,
    Location = 6,
    File = 7,
    Transfer = 8,
    Other = 10,
}

impl TryFrom<i16> for MessageType {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageType::Text),
            2 => Ok(MessageType::Image),
            3 => Ok(MessageType::Voice),
            4 => Ok(MessageType::Video),
            5 => Ok(MessageType::LivePhoto),
            6 => Ok(MessageType::Location),
            7 => Ok(MessageType::File),
            8 => Ok(MessageType::Transfer),
            10 => Ok(MessageType::Other),
            n if (9..=9).contains(&n) => Ok(MessageType::Other),
            _ => Err("消息类型必须在 1..10 之间"),
        }
    }
}

////////

/// # [COMMAND] - 评论发送命令
/// * `desc`: `消息类型: 1 文字 2 图像 3 语音 4 视频 5 livephoto 6 位置 7 文件 8 转账...`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentCommand {
    pub _id: Option<String>,         // 客户端 UUID v4（用于防重）
    pub user_id: i64,                // 用户 ID
    pub video_id: i64,               // 视频 ID
    pub parent_id: Option<i64>,      // 父评论 ID
    pub message_type: i16,           // 消息类型 (1..10)
    pub content: String,             // 内容
    pub media_ids: Option<Vec<i64>>, // 关联的媒体文件 (可选，因为有时评论不带)
    pub photos_url: Option<String>,  // 照片 URL (老系统兼容，逗号分隔)
    pub video_url: Option<String>,   // 视频 URL (老系统兼容，逗号分隔)
    pub voice_url: Option<String>,   // 语音 URL (老系统兼容，逗号分隔)
}

/// # [BUILD] - 构造函数
impl CommentCommand {
    //

    ////////

    /// # [FROM] - 实体转换
    /// * 将当前的 Command 转换为核心领域的评论实体 (Entity)
    /// * 同时从外部（用例/业务上下文）动态注入实时的真实 uid 和 video_id
    pub fn into_entity(self, real_user_id: i64, real_video_id: i64) -> VideoCommentEntity {
        // 1. 获取当前 UTC 时间与时间戳
        let now = Utc::now();
        let now_ts = now.timestamp();

        // 2. 校验消息类型 (1..10)，不合法则默认存为 1 (文本)
        let validated_type = MessageType::try_from(self.message_type)
            .map(|t| t as i16)
            .unwrap_or(1);

        // 3. 兼容老系统：自动从前端传来的 payload 中提取图片 URL 用逗号拼接
        // 注：若后续 payload 结构有调整，可按需适配 self.media_ids 或 payload
        let legacy_photos_url = self.photos_url;

        // 4. 客户端ID
        let client_id = self._id.as_deref().and_then(|value| {
            let uuid = uuid::Uuid::parse_str(value.trim()).ok()?;
            (uuid.get_version() == Some(uuid::Version::Random)).then(|| uuid.to_string())
        });

        // 视频评论实体表
        VideoCommentEntity {
            _id: client_id.or_else(|| Some(uuid::Uuid::new_v4().to_string())),
            user_id: real_user_id,         // 用户 ID
            video_id: real_video_id,       // 视频 ID
            parent_id: self.parent_id,     // 父评论 ID
            message_type: validated_type,  // 对应实体中的字段，如需同步改名字可在 entity 中调整
            content: self.content,         // 内容
            photos_url: legacy_photos_url, // 照片URL (兼容旧版)
            video_url: self.video_url,     // 视频URL (兼容旧版)
            voice_url: self.voice_url,     // 语音URL (兼容旧版)
            likes: 0,                      // 点赞数量
            dislikes: 0,                   // 不喜欢数量
            collects: 0,                   // 收藏数量
            reply: 0,                      // 回复数量
            visibility: 5,                 // 修正为默认所有人可见 (对应注释)
            add_time: now_ts,              // 添加时间(兼容旧版)
            created_at: Some(now),         // 创建时间
            updated_at: Some(now),         // 更新时间
            ..Default::default()           // 其他默认
        }
    }
}

//////// END
