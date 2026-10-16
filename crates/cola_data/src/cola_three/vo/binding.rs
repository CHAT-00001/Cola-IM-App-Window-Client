// cola_data/src/cola_three/vo/binding.rs  -- 绑定 VO
// 2026/6/18 04:10

////////

use crate::cola_three::info::binding::BindingInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 业务绑定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BindingVo {
    pub id: i64,
    pub three_config_id: i64,
    pub biz_module: String,
    pub biz_type: String,
    pub status: i16,
}

// 转换函数
impl From<BindingInfo> for BindingVo {
    fn from(info: BindingInfo) -> Self {
        Self {
            id: info.id,
            three_config_id: info.three_config_id,
            biz_module: info.biz_module,
            biz_type: info.biz_type,
            status: info.status,
        }
    }
}

//////// END
