// cola_dynamic/address.rs  -- MARKET - Command  - 商店申请
// 2026/6/18 13:08

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 地址簿创建命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddressCommand {
    pub name: String,
    pub country: String,
    pub province: String,
    pub city: String,
    pub area: String,
    pub address: String,
    pub phone: String,
    pub is_default: bool,
}

impl AddressCommand {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("收件人姓名不能为空"));
        }
        if self.phone.trim().len() < 7 {
            return Err(anyhow::anyhow!("请填写有效的联系电话"));
        }
        if self.address.trim().is_empty() {
            return Err(anyhow::anyhow!("详细地址不能为空"));
        }
        Ok(())
    }
}
