// cola_data/src/cola_three/info/binding  -- THREE - 绑定信息
// 2026/6/18

use crate::cola_three::entity::binding::ThreeBizBindingEntity;
use serde::{Deserialize, Serialize};

//////

/// # [INFO] - 业务绑定信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BindingInfo {
    pub id: i64,
    pub three_config_id: i64,
    pub biz_module: String,
    pub biz_type: String,
    pub status: i16,
}

impl From<ThreeBizBindingEntity> for BindingInfo {
    fn from(e: ThreeBizBindingEntity) -> Self {
        Self {
            id: e.id,
            three_config_id: e.three_config_id,
            biz_module: e.biz_module,
            biz_type: e.biz_type,
            status: e.status,
        }
    }
}
