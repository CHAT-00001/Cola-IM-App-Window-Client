// cola_data/src/cola_im/cmd/message.rs  -- IM - Command - 消息
// 2026-07-07

//////

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 消息 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCommand {
    pub send_id: Option<String>, // 发送 ID
    pub user_id: i64,            // 用户 ID
    pub at_id: i64,              // 父消息 ID
    pub channel_id: i16,         // 通道
    pub content: String,         // 内容
    pub color: Option<String>,   // 颜色
    pub mode: i16,               // 模式
    pub duration: i16,           // 留存时间
    pub status: i16,             // 状态码
}

////// END
