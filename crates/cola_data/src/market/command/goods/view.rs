// data/src/market/cmd/goods/view.rs
// data - MARKET - cmd - 商品 - 浏览记录指令
// 2026/8/11 22:08 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [DTO] - 商品浏览记录 - 创建指令 (CMD)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoodsViewCreateCommand {
    pub uid: i64,
    pub goods_id: i64,
    pub client_ip: Option<String>,
    pub client_type: Option<i16>,
    pub device_os: Option<String>,
    pub device_model: Option<String>,
    pub user_agent: Option<String>,
    pub gps_lat: Option<f64>,
    pub gps_lng: Option<f64>,
    pub gps_addr: Option<String>,
}

//////// END
