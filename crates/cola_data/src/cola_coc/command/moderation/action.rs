// data/src/cola_coc/cmd/moderation/action.rs -- 数据 - COC - cmd - 治理 - 动作
// 2026/9/1 05:27 Created.

////////


// .rs --
// 2026/5/19 22:13 by wx: cestbon10080

////////

use crate::cola_video::entity::danmaku::DanmakuEntity;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 运营中心 - 治理命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModerationCommand {
    pub user_id: i64,          // 用户 ID
    pub video_id: i64,         // 视频 ID
    pub danmaku_type: i16,     // 弹幕类型: 0. 滚动 1. 固定底部 2. 固定
    pub channel_id: i16,       // 通道
    pub content: String,       // 内容
    pub play_time: i32,        // 播放时间（ms）
    pub duration: i16,         // 停留时长
    pub color: Option<String>, // 颜色（可选）
    pub mode: i16,             // 位置
    pub add_time: i32,         // 创建时间
    pub visibility: i16, // 可见范围； 0. 不可见 1. 自己 2. 朋友 3. 粉丝 4. 所有人（黑名单除外）
    pub status: i16,     // 状态: 0. 审核 1. 可见
}

/// # [BUILD] - 构造新弹幕
impl ModerationCommand {
    /// 将当前的 Command 转换为核心领域的实体 (Entity)
    /// 同时支持从外部注入实时的 uid 和 video_id
    pub fn into_entity(self, real_uid: i64, real_video_id: i64) -> DanmakuEntity {
        // 这里的创建时间直接取当前系统时间戳
        let now_ts = chrono::Local::now().timestamp();

        // 🚀 处理 duration 默认值：如果未传或为 0，则默认 5000 毫秒
        let final_duration = if self.duration == 0 {
            5000
        } else {
            self.duration
        };

        // 🚀 处理 color 默认值：如果为 None 或空字符串，则默认 "#FFFFFF"
        let final_color = match self.color {
            Some(ref c) if !c.is_empty() => self.color,
            _ => Some("#FFFFFF".to_string()),
        };

        DanmakuEntity {
            user_id: real_uid,           // 用户 ID
            video_id: real_video_id,     // 视频 ID
            channel_id: self.channel_id, // 默认 1
            content: self.content,       // 不能为空
            play_time: self.play_time,   // 发射时间
            duration: final_duration,    // 🚀 默认 5000 毫秒
            color: final_color,          // 🚀 默认 "#FFFFFF"
            mode: self.mode,
            likes: 0,      // 初始计数为 0
            dislikes: 0,   // 初始计数为 0
            visibility: 5, // 默认所有人可见
            status: self.status,
            sync_time: now_ts,
            ..Default::default()
        }
    }
}

//////// END
