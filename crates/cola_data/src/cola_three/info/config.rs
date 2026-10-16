// cola_data/src/cola_three/info/config  -- THREE - 配置信息
// 2026/6/18

use crate::cola_three::entity::config::ThreeConfigEntity;
use serde::{Deserialize, Serialize};

//////

/// # [INFO] - 第三方服务配置信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigInfo {
    pub id: i64,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    // secret_key 不暴露到 info
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<ThreeConfigEntity> for ConfigInfo {
    fn from(e: ThreeConfigEntity) -> Self {
        Self {
            id: e.id,
            type_id: e.type_id,
            vendor_id: e.vendor_id,
            name: e.name,
            bucket: e.bucket,
            access_key: e.access_key,
            endpoint: e.endpoint,
            region: e.region,
            config_json: e.config_json,
            remark: e.remark,
            status: e.status,
        }
    }
}
