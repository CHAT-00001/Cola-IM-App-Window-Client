// market/entity/shop/count.rs
// 数据 - MARKET - entity - 商店 - 计数表
// 2026/8/3 22:50 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商店 - 计数表
/// * `pg schema`: `market`
/// * `table name`: `shop_count`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopCountEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 操作者用户ID
    pub name: String,                      // 收件人名称
    pub country: String,                   // 国家/地区
    pub province: String,                  // 省/州
    pub city: String,                      // 城市
    pub area: String,                      // 区县
    pub address: String,                   // 详细地址(不可为空)
    pub area_code: String,                 // 地区号
    pub phone: String,                     // 电话号码
    pub is_default: i16,                   // 是否默认
    pub id_del: i16,                       // 是否删除(兼容旧版PHP, 默认0.否)
    pub id_deleted: Option<bool>,          // 是否删除: (默认false, 否)
    pub add_time: i32,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i32,                     // 更新时间（兼容旧版PHP）
    pub create_at: Option<DateTime<Utc>>,  // 创建时间
    pub update_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间(软删除)
}

////////

/// # [COLUMNS] - 地址表
pub const ADDRESS_COLUMNS: &'static str = r#"
        id, uid, name, country, province, city, area,
        address, area_code, phone, is_default,
        add_time, upd_time, create_at, update_at, id_del, deleted_at
    "#;

//////// END
