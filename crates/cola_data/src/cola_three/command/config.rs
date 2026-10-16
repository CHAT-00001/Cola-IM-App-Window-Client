// cola_data/src/cola_three/cola_dynamic/config  -- THREE - 配置命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 新增/更新第三方服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertConfigCommand {
    pub id: Option<i64>,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}
