// cola_data/src/cola_im/cmd/setting.rs  -- IM - Command - 聊天设置
// 2026-07-07

//////

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 聊天设置命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSettingCommand {
    pub user_id: i64,               // 用户 ID
    pub chat_id: i64,               // 聊天 ID
    pub is_muted: bool,             // 是否静音
    pub is_pinned: bool,            // 是否置顶
    pub is_blocked: bool,           // 是否屏蔽
    pub background: Option<String>, // 聊天背景
    pub remark: Option<String>,     // 备注
}

////// END
