// /shop.rs
//
// 2026/8/3 22:52 Created.

////////

// cola_data/src/market/handler/shop_apply.rs  -- 可乐数据中心 - MARKET - handler - 商店
// 2026/6/18 10:42

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商店表
/// * table name: shop_apply
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopApplyEntity {
    pub id: i64,                          // id
    pub uid: i64,                         // 操作者用户ID
    pub market_id: i64,                   // 市场ID
    pub name: String,                     // 中文名称
    pub name_en: String,                  // 英文名称
    pub thumb: String,                    // 封面
    pub des: i64,                         // 简介
    pub username: i64,                    // 联系人名称
    pub cardno: i64,                      // 身份证号
    pub contact: String,                  // 联系人
    pub country_code: String,             // 地区代码
    pub phone: i32,                       // 电话
    pub province: String,                 // 省份
    pub city: String,                     // 市
    pub area: String,                     // 地区
    pub address: String,                  // 详细地址
    pub service_phone: i32,               // 客服电话
    pub receiver: i32,                    // 退货收货人
    pub receiver_phone: i16,              // 退货人联系电话
    pub receiver_province: i32,           // 退货人省份
    pub receiver_city: String,            // 退货人市
    pub receiver_area: i16,               // 退货人地区
    pub receiver_address: i16,            // 退货人详细地址
    pub license: Option<String>,          // 许可证
    pub certificate: Option<String>,      // 营业执照
    pub other: Option<String>,            // 其他证件
    pub reason: Option<String>,           // 审核中文原因
    pub reason_en: String,                // 审核英文原因
    pub order_percent: Option<String>,    // 订单分成比例
    pub sale_nums: Option<String>,        // 店铺总销量
    pub quality_points: f32,              // 质量分 0.1-5.0
    pub service_points: f32,              // 服务分 0.1-5.0
    pub express_points: f32,              // 物流分 0.1-5.0
    pub shipment_overdue_num: i32,        // 店铺逾期发货次数
    pub lat: String,                      // 纬度
    pub lng: String,                      // 经度
    pub views: i32,                       // 标签ID
    pub collects: i32,                    // 被收藏数量
    pub shares: i32,                      // 被分享数量
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
