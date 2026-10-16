// cola_data/src/cola_im/cmd/contact_request.rs  -- IM - Command - 联系人添加请求
// 2026-07-07

//////

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 联系人 - 添加请求命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactRequestCommand {
    pub send_id: String,         // 发送 ID
    pub owner_id: i64,           // 用户 ID
    pub card_id: i64,            // 对方名片 ID
    pub message: Option<String>, // 验证消息
    pub is_refused: Option<i16>, // 是否拒绝
    pub replay: Option<String>,  // 回复消息
    pub sort: Option<i64>,       // 排序
    pub status: i16,             // 状态码
}

////// END
