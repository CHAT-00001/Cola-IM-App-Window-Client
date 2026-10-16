// cola_data/src/cola_im/commandc/contact.rs  -- IM - Command - 联系人
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 联系人 - 添加命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactCommand {
    pub send_id: String,             // 发送 ID
    pub owner_id: i64,               // 用户 ID
    pub group_id: Option<i64>,       // 分组 ID
    pub card_id: i64,                // 对方名片 ID
    pub remark_name: Option<String>, // 备注名称
    pub is_stared: i16,              // 是否星标: 默认0否
    pub favorites: bool,             // 特别关心
    pub blocked: bool,               // 是否已拉黑名单
    pub deleted: bool,               // 是否已删除
    pub sort: Option<i64>,           // 排序
    pub status: i16,                 // 状态码
}
