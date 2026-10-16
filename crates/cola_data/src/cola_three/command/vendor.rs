// cola_data/src/cola_three/cola_dynamic/vendor  -- THREE - 厂商命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 新增/更新厂商
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertVendorCommand {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}
