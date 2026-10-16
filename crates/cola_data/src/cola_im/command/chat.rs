// cola_data/src/cola_im/cmd/chat.rs  -- 数据中心 - IM - Command - 聊天
// 2026-07-07

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 聊天 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatCommand {
    pub user_id: i64,           // 用户 ID
    pub title: String,          // 聊天标题
    pub chat_type: i16,         // 聊天类型 1:单聊 2:群聊
    pub member_ids: Vec<i64>,   // 成员IDs
    pub avatar: Option<String>, // 聊天头像
    pub status: i16,            // 状态码
}

//////// END
