// cola_data/src/cola_three/cola_dynamic/binding  -- THREE - 绑定命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 新增/更新业务绑定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertBindingCommand {
    pub id: Option<i64>,
    pub three_config_id: i64,
    pub biz_module: String, // video / user / cola_live / music
    pub biz_type: String,   // node / sms / stream
    pub status: i16,
}
