// market/entity/shop/rating.rs
// 数据中心 - MARKET - entity - 商店 评分表
// 2026/8/3 22:55 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商店 - 评分表
/// * `pg schema`: `market`
/// * `table name`: `shop_ratingt`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopRatingEntity {
    pub id: i64,                           // id
    pub uuid: Option<String>,              // 操作者用户ID
    pub service_rating: f32,               // 服务评分: (默认5.0)
    pub express_rating: f32,               // 物流评分: (默认5.0)
    pub status: i16,                       // 状态码
    pub id_del: i16,                       // 是否删除(兼容旧版PHP, 默认0.否)
    pub id_deleted: Option<bool>,          // 是否删除: (默认false, 否)
    pub add_time: i32,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i32,                     // 更新时间（兼容旧版PHP）
    pub create_at: Option<DateTime<Utc>>,  // 创建时间
    pub update_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间(软删除)
}

////////

/// # [COLUMNS] - 商店评分表
/// * `desc`: 商店评分
pub const SHOP_RATING_COLUMNS: &'static str = r#"
        id, uuid, name, service_rating, express_rating,
        status, id_del,id_deleted,
        add_time, upd_time,
        create_at, update_at, deleted_at
    "#;

//////// END
