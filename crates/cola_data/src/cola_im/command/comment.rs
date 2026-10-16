// cola_data/src/new/cola_dynamic/add  -- VIDEO - Command - 评论
// 2026/5/20 12:01

////////

use crate::video::entity::comment::VideoCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum CommentType {
    Text = 1,
    Image = 2,
    LivePhoto = 3,
    Video = 4,
    Voice = 5,
    Other = 10,
}

impl TryFrom<i16> for CommentType {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CommentType::Text),
            2 => Ok(CommentType::Image),
            3 => Ok(CommentType::LivePhoto),
            4 => Ok(CommentType::Video),
            5 => Ok(CommentType::Voice),
            n if n >= 1 && n <= 10 => Ok(CommentType::Other),
            _ => Err("评论类型必须在 1..10 之间"),
        }
    }
}

/// # [COMMAND] - 评论发送命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentCommand {
    pub user_id: i64,               // 用户 ID
    pub video_id: i64,              // 视频 ID
    pub parent_id: Option<i64>,     // 爸爸评论 ID
    pub comment_type: i16,          // 类型 (1..10)
    pub content: String,            // 内容
    pub payload: MediaInfo,         // 负载：支持多图与LivePhoto混发
    pub photos_url: Option<String>, // 照片 URL (老系统习惯，存逗号分隔的多图字符串)
    pub video_url: Option<String>,  // 视频 URL
    pub voice_url: Option<String>,  // 语音 URL
    pub likes: i32,                 // 喜欢数量
    pub dislikes: i32,              // 不喜欢数量
    pub collects: i32,              // 收藏数量
    pub visibility: i16,            // 可见范围：5所有人可见
    pub add_time: i64,              // 创建时间（兼容旧版PHP）
    pub created_at: DateTime<Utc>,  // 创建时间
    pub updated_at: DateTime<Utc>,  // 更新时间
}

/// # [BUILD] - 构造函数
impl CommentCommand {
    /// 将当前的 Command 转换为核心领域的评论实体 (Entity)
    /// 同时从外部（用例/业务上下文）动态注入实时的真实 uid 和 video_id
    pub fn into_entity(self, real_uid: i64, real_video_id: i64) -> VideoCommentEntity {
        // 1. 获取当前 UTC 时间戳
        let now = Utc::now().timestamp();
        let now_ts = Utc::now();

        // 2. 校验评论类型 (1..10)，不合法则默认存为 1 (文本)
        let validated_type = CommentType::try_from(self.comment_type)
            .map(|t| t as i16)
            .unwrap_or(1);

        // 3. 兼容老系统：自动从前端传来的 payload 中提取图片 URL 用逗号拼接
        let legacy_photos_url = if self.payload.items.is_empty() {
            None
        } else {
            let urls: Vec<String> = self
                .payload
                .items
                .iter()
                .map(|item| match item {
                    MediaItem::Photo { url } => url.clone(),
                    MediaItem::LivePhoto { image_url, .. } => image_url.clone(),
                })
                .collect();
            Some(urls.join(",")) // 经典 PHP 时代的逗号分隔存储
        };

        VideoCommentEntity {
            user_id: real_uid,       // 动态注入
            video_id: real_video_id, // 动态注入
            parent_id: self.parent_id,
            comment_type: validated_type,
            content: self.content,
            // 💡 提示：如果你的库里有存放复杂结构体的 payload 字段，直接赋值。如果没有该字段，Entity 尾部的 ..Default::default() 会安全消化
            // payload: self.payload,
            photos_url: legacy_photos_url,
            video_url: self.video_url,
            voice_url: self.voice_url,

            // 核心业务初始默认值规则
            likes: 0,
            dislikes: 0,
            collects: 0,
            visibility: 4, // 默认所有人可见

            add_time: now,
            created_at: Option::from(now_ts),
            updated_at: Option::from(now_ts),
            ..Default::default()
        }
    }
}

/// 单个媒体条目：支持普通照片和实况照片并存
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "cola_data")]
pub enum MediaItem {
    /// 普通照片：只需传入图片 URL
    Photo { url: String },
    /// 实况照片：包含一张静态图和一段短视频
    LivePhoto {
        image_url: String,
        video_url: String,
    },
}

/// 媒体负载信息：支持多种媒体并行（如：3张普通照片 + 1张LivePhoto）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaInfo {
    pub items: Vec<MediaItem>,
}

impl MediaInfo {
    /// 便捷构造器：创建一个空的媒体负载
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 追加一个媒体文件
    pub fn push(&mut self, item: MediaItem) {
        self.items.push(item);
    }
}

//////// END
