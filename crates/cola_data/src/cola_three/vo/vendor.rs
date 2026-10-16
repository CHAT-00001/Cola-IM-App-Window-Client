// cola_data/src/cola_three/vo/vendor2.rs  -- 数据中心 - 第三方 - vo - 厂商
// 2026/6/18 10:11

////////

use crate::cola_three::info::vendor::VendorInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 厂商
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VendorVo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

// 构造函数
impl From<VendorInfo> for VendorVo {
    fn from(info: VendorInfo) -> Self {
        Self {
            id: info.id,
            code: info.code,
            name: info.name,
            sort: info.sort,
            status: info.status,
        }
    }
}

//////// END
