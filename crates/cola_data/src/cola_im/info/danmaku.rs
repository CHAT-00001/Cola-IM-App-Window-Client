// info/danmaku.rs  -- 数据中心 - VIDEO - 信息 - 弹幕信息
// 2026/5/22 16:10

////////

use serde::{Deserialize, Serialize};
use crate::user::info::user::UserInfo;
use crate::video::entity::danmaku::DanmakuEntity;

////////

/// # [INFO] - 弹幕信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DanmakuInfo {
    pub id: i64,                 // 弹幕 ID
    pub send_id: Option<String>, // 发送 ID （客户端生成，用于去重）
    pub user_id: i64,            // 用户 ID
    pub video_id: i64,           // 视频 ID
    pub content: String,         // 内容
    pub likes: i32,              // 点赞数量
    pub dislikes: i32,           // 不喜欢数量
    pub color: String,           // 🚀 颜色改成了 String 类型 (不为空，默认 "#FFFFFF")
    pub mode: i16,               // 模式：0.滚动 1.顶部固定 2.底部固定 3.高级/代码弹幕
    pub play_time: i32,          // 弹幕在视频中的渲染时间点 (单位：毫秒)
    pub duration: i16,           // 弹幕在屏幕上的留存展示时间 (单位：毫秒)
    pub send_time: i64,          // 发送时间（毫秒级时间戳）
    pub sync_time: i64,          // 入库同步时间
}

////////

/// # 构造函数
impl DanmakuInfo {

    pub fn from_entity(
        entity: DanmakuEntity,
    ) -> Self {

        Self {
            id: entity.id,
            send_id: entity.send_id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            content: entity.content,
            likes: entity.likes,
            dislikes: entity.dislikes,
            color: entity.color.unwrap_or_else(|| "#FFFFFF".to_string()),
            mode: entity.mode,
            play_time: entity.play_time,
            duration: entity.duration,
            send_time: entity.send_time,
            sync_time: entity.sync_time,
        }
    }

    /// 构造全新的弹幕信息，并自动判定是否为视频作者
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        send_id: Option<String>,
        user_id: i64,
        video_id: i64,
        content: String,
        likes: i32,
        dislikes: i32,
        color: Option<String>,
        mode: Option<i16>,
        play_time: i32,
        duration: Option<i16>,
        send_time: i64,
        sync_time: i64,
        video_author_id: i64,
    ) -> Self {
        let is_author = user_id == video_author_id;

        Self {
            id,
            send_id,
            user_id,
            video_id,
            content,
            likes,
            dislikes,
            color: color.unwrap_or_else(|| "#FFFFFF".to_string()), // 🚀 完美对齐 String 类型
            mode: mode.unwrap_or(0),
            play_time,
            duration: duration.unwrap_or(5000),
            send_time,
            sync_time,
        }
    }
}

//////// END
