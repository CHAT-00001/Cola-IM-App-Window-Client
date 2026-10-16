// data/src/market/entity/goods/view.rs
// 数据 - MARKET - entity - 商品 - 浏览记录表
// 2026/8/3 23:03 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::net::IpAddr;

////////

/// # [ENTITY] - 市场 - 商品 - 浏览记录表
/// * `pg schema`: `market`
/// * `table name`: `goods_visit`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsViewEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub uuid: Option<String>,              // UUID v4
    pub uid: i64,                          // 用户 ID
    pub goods_id: i64,                     // 商品 ID
    pub client_ip: Option<String>,         // 原始访问IP（IpAddr转字符串存储，用于解析归属地）
    pub ip_area: Option<String>,           // IP归属地 例：广西-南宁-电信
    pub gps_lat: Option<f64>,              // GPS 纬度
    pub gps_lng: Option<f64>,              // GPS 经度
    pub gps_addr: Option<String>,          // 逆地理编码后的详细地址：广西南宁市青秀区XX街道
    pub user_agent: Option<String>,        // 完整UA请求头
    pub client_type: Option<i16>,          // 客户端类型：1 H5 2 安卓APP 3 iOS APP 4 小程序
    pub device_os: Option<String>,         // 设备系统：Android 14 / iOS 18 / Windows
    pub device_model: Option<String>,      // 设备型号：iPhone 15 / XiaoMi 14
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub is_deleted: Option<bool>,          // 是否删除 (逻辑删除)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 商品浏览记录表全部字段
/// * `desc`: `SQLx 映射`
pub const GOODS_VIEW_COLUMNS: &'static str = r#"
        id, uuid, uid, goods_id, client_ip, ip_area,
        gps_lat, gps_lng, gps_addr, user_agent, client_type,
        device_os, device_model, status, add_time, upd_time,
        is_deleted, created_at, updated_at, deleted_at
    "#;

//////// END
