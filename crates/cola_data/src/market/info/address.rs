// info/address.rs  -- 信息层 - 地址簿
// 2026/6/18 12:16

////////

mod address;

use crate::market::entity::address::AddressEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 地址信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub id: i64,
    pub uid: i64,
    pub name: String,
    pub full_address: String, // 组合后的完整地址
    pub phone: String,
    pub is_default: bool,
}

// 构造函数
impl AddressInfo {
    pub fn not_found() -> Self {
        Self {
            id: 0,
            uid: 0,
            name: "未知收件人".to_string(),
            full_address: "地址信息不存在".to_string(),
            phone: "000-0000-0000".to_string(),
            is_default: false,
        }
    }
}

impl From<AddressEntity> for AddressInfo {
    fn from(e: AddressEntity) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            name: e.name,
            // 这里假设你的业务层会把 ID 转换成具体名称，或者直接拼接 address
            full_address: e.address,
            phone: e.phone,
            is_default: e.is_default == 1,
        }
    }
}
