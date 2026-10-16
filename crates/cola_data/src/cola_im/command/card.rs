// cola_data/src/cola_im/cmd/card.rs  -- IM - Command - 名片
// 2026-07-07

//////

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 名片命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardCommand {
    pub user_id: i64,                // 用户 ID
    pub video_id: i64,               // 视频 ID
    pub parent_id: Option<i64>,      // 父评论
    pub contact_type: i16,           // 联系人类型
    pub content: String,             // 内容
    pub photos_url: Option<String>,  // 照片 url
    pub video_url: Option<String>,   // 视频 url
    pub voice_url: Option<String>,   // 语音 url
    pub duration: Option<i32>,       // 时长
    pub first_name: String,          // 姓
    pub last_name: String,           // 名
    pub visibility: i16,             // 可见范围
    pub region_code: Option<String>, // 地区码
    pub status: i16,                 // 状态
}

////// END
