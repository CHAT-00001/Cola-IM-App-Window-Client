// cola_data/src/cola_im/cmd/contact_relation.rs  -- IM - Command - 联系人关系
// 2026-07-07

//////

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 联系人 - 关系命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactRelationCommand {
    pub name: String,         // 名称
    pub name_zh: String,      // 中文名称
    pub icon: Option<String>, // 关系图标
    pub sort: Option<i64>,    // 排序
    pub status: i16,          // 状态码
}

////// END
