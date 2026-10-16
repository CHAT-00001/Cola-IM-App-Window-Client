// data/src/market/info/goods/view.rs
// 数据 - MARKET - info - 商品 - 浏览记录详情
// 2026/8/11 22:09 Created.

////////

use crate::market::entity::goods::view::GoodsViewEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 商品浏览记录 - 详情响应 (INFO)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoodsViewInfo {
    pub id: i64,                           // 浏览 ID
    pub uuid: Option<String>,              // UUID
    pub uid: i64,                          // 浏览者 ID
    pub goods_id: i64,                     // 商品 ID
    pub client_ip: Option<String>,         // 客户端 IP
    pub ip_area: Option<String>,           // IP 属地
    pub client_type: Option<i16>,          // 客户端类型
    pub device_os: Option<String>,         // 设备系统
    pub device_model: Option<String>,      // 设备型号
    pub status: i16,                       // 状态码
    pub created_at: Option<DateTime<Utc>>, // 创建时间
}

// -----------------------------------------------------------------
// 类型转换实现 (Entity -> Info)
// -----------------------------------------------------------------

impl From<GoodsViewEntity> for GoodsViewInfo {
    fn from(entity: GoodsViewEntity) -> Self {
        Self {
            id: entity.id,
            uuid: entity.uuid,
            uid: entity.uid,
            goods_id: entity.goods_id,
            client_ip: entity.client_ip,
            ip_area: entity.ip_area,
            client_type: entity.client_type,
            device_os: entity.device_os,
            device_model: entity.device_model,
            status: entity.status,
            created_at: entity.created_at,
        }
    }
}

//////// END
