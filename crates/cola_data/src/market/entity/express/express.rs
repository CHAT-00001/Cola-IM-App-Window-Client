// market/entity/express/express.rs
// 数据中心 - MARKET - entity - 快递 - 物流公司表
// 2026/8/3 22:40 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 快递公司表
/// * `pg schema`: `market`
/// * `table name`: `shop_express`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ExpressEntity {
    pub id: i64,                           // id
    pub uuid: Option<String>,              // uuid v4
    pub uid: i64,                          // 作者ID
    pub express_name: String,              // 名称
    pub name_en: String,                   // 英文名称
    pub express_phone: String,             // 电话
    pub express_thumb: String,             // 封面
    pub express_status: i16,               // 状态: 0. 禁用  1. 启用
    pub express_code: String,              // 代码
    pub sort: i16,                         // 排序: 默认9999
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub list_order: i32,                   // 老版排序
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 更新时间
}

//////// END
