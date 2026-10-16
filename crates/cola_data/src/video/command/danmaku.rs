// cola_data/src/video/cmd/danmaku.rs -- 数据 - VIDEO - cmd - 弹幕 - mod
// 2026/5/19 22:13 by wx: cestbon10080

////////

use crate::video::entity::danmaku::DanmakuEntity;
use crate::common::kits::snow::next_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version};

////////

/// # [COMMAND] - 视频 -弹幕发布命令
/// * `desc`: `简单的发布命令`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DanmakuCommand {
    pub _id: Option<String>,     // 客户端 UUID v4，用于乐观发布和离线同步
    pub video_id: i64,           // 视频 ID
    pub channel_id: Option<i32>, // 通道（可选，不传默认 1）
    pub at: Option<String>,      // 艾特内容摘要
    pub content: String,         // 内容
    pub play_time: i32,          // 播放时间（ms）
    pub duration: i16,           // 停留时长
    pub color: Option<String>,   // 颜色（可选）
    pub mode: i16,               // 位置:  1.滚动 2.固定
    pub send_time: Option<i64>,  // 发送时间(可选)
    pub status: Option<i16>,     // 状态: 0. 审核 1. 可见（可选，不传默认 1）
}

/// # [BUILD] - 构造新弹幕
impl DanmakuCommand {
    //

    ////////

    /// # [FROM] - 转换
    /// * 将当前的 Command 转换为核心领域的实体 (Entity)
    /// * 同时支持从外部注入实时的 uid 和 video_id
    pub fn into_entity(self, real_uid: i64, real_video_id: i64) -> DanmakuEntity {
        // 获取当前时间（服务端统一生成）
        let now: DateTime<Utc> = Utc::now();
        let now_ts = now.timestamp(); // i64时间戳

        // 🚀 1. 生成雪花 ID (snowflake 库)
        let generated_id = next_id(1);

        // 🚀 2. 默认频道：如果未传，则默认补 1
        let final_channel = match self.channel_id {
            Some(id) if id > 0 => id,
            _ => 1,
        };

        // 🚀 3. 默认状态：如果未传，则默认补 1（可见）
        let final_status = match self.status {
            Some(s) => s,
            _ => 1,
        };

        // ✅ 客户端 UUID v4 有效时保留；缺失、为空或格式错误时由服务端补齐
        let client_id = self._id.as_deref().and_then(|value| {
            let uuid = Uuid::parse_str(value.trim()).ok()?;
            (uuid.get_version() == Some(Version::Random)).then(|| uuid.to_string())
        });
        let final_client_id = client_id.or_else(|| Some(Uuid::new_v4().to_string()));

        // 🚀 4. 处理 duration 默认值：如果未传或为 0，则默认 5000 毫秒
        let final_duration = if self.duration == 0 {
            5000
        } else {
            self.duration
        };

        // 🚀 5. 处理 color 默认值：如果为 None 或空字符串，则默认 "#FFFFFF"
        let final_color = match self.color {
            Some(ref c) if !c.is_empty() => self.color,
            _ => Some("#FFFFFF".to_string()),
        };

        // 弹幕实体表结构
        DanmakuEntity {
            id: generated_id,          // 🚀 注入服务端生成的雪花 ID
            _id: final_client_id,      // ✅ 客户端 UUID v4，无效时使用服务端 UUID v4
            user_id: real_uid,         // 🚀 使用外部传入的当前用户 ID
            video_id: real_video_id,   // 视频 ID
            channel_id: final_channel, // 🚀 自动兜底为 1
            content: self.content,     // 不能为空
            play_time: self.play_time, // 播放器时间
            duration: final_duration,  // 🚀 默认 5000 毫秒
            color: final_color,        // 🚀 默认 "#FFFFFF"
            mode: self.mode,           // 模式
            likes: 0,                  // 初始计数为 0
            dislikes: 0,               // 初始计数为 0
            collects: 0,               // 初始计数为 0
            visibility: 5,             // 5 默认所有人可见
            status: final_status,      // 🚀 自动兜底为 1（可见）
            sync_time: now_ts,         // 发送时间戳
            created_at: now,           // 🚀 服务端统一生成的当前 UTC 时间
            ..Default::default()
        }
    }
}

//////// END
