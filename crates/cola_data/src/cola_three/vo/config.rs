// cola_data/src/threel/vo/config.rs  -- 数据中心 - 通用 - vo - config配置
// 2026/6/18 10:12

////////

use crate::cola_three::info::config::ConfigInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 第三方服务配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigVo {
    pub id: i64,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

// 转换函数
impl From<ConfigInfo> for ConfigVo {
    fn from(info: ConfigInfo) -> Self {
        Self {
            id: info.id,
            type_id: info.type_id,
            vendor_id: info.vendor_id,
            name: info.name,
            bucket: info.bucket,
            access_key: info.access_key,
            endpoint: info.endpoint,
            region: info.region,
            config_json: info.config_json,
            remark: info.remark,
            status: info.status,
        }
    }
}

//////// END
