// cola_data/src/market/handler/order  -- 可乐数据中心 - MARKET - handler - 订单
// 2026/6/18 11:05

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 订单表
/// * table name: shop_order
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopOrderEntity {
    pub id: i64,                      // 订单ID
    pub _id: Option<i64>,             // 雪花ID (可选)
    pub line_id: Option<i64>,         // 状态流水线ID
    pub line_station_id: Option<i64>, // 流水线站点ID
    pub order_type: i64,              // 订单类型：0. 虚拟 1.邮寄 2. 自取
    pub shop_uid: String,             // 卖家用户ID
    pub goods_id: i64,                // 商品id
    pub goods_name: String,           // 商品名称
    pub spec_name: String,            // 商品规格ID
    pub spec_thumb: String,           // 规格封面
    pub nums: i32,                    // 购买数量
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal, // 单价
    #[serde(with = "rust_decimal::serde::str")]
    pub total: Decimal, // 总价
    pub username: String,             // 客户名称
    pub phone: String,                // 客户电话
    pub country: String,              // 国家/地区
    pub country_code: String,         // 地区代码
    pub province: String,             // 省州
    pub city: String,                 // 城市
    pub area: String,                 // 区县
    pub address: String,              // 详细地址
    pub postage: String,              // 邮费
    pub orderno: String,              // 订单编号
    // * 1 支付宝 2 微信 3 余额 4 微信小程序 5 paypal 6 Braintree_paypal
    #[serde(rename = "type")]
    pub r#type: i16, // 支付类型
    // *  -1 已关闭  0 待付款 1 待发货 2 待收货 3 待评价 4 已评价 5 退款
    pub status: i16,             // 订单状态
    pub trade_no: String,        // 三方订单号
    pub cancel_time: i32,        // 取消时间
    pub paytime: i32,            // 付款时间
    pub shipment_time: i32,      // 订单编号
    pub receive_time: i32,       // 收货时间
    pub evaluate_time: i32,      // 评价时间
    pub settlement_time: i32,    // 结算时间（款项打给卖家）
    pub is_append_evaluate: i16, // 是否可追加评价
    pub order_percent: i32,      // 抽成比例
    pub refund_starttime: i32,   // 发起退款时间
    pub refund_endtime: i32,     // 订单退款处理结束时间
    pub refund_status: i16,      // 退款处理结果 -2取消申请 -1 失败 0 处理中 1 成功
    pub refund_shop_result: i16, // 卖家处理结果 0 未处理 -1 拒绝 1 同意
    pub express_name: String,    // 物流名称
    pub express_name_en: String, // 物流英文名称
    pub express_phone: String,   // 物流公司电话
    pub express_thumb: i16,      // 物流公司缩略图
    pub express_code: i32,       // 物流公司代号
    pub express_number: String,  // 物流单号
    pub message: String,         // 买家留言内容
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal, // 平台自营商品设置的代售佣金
    pub liveuid: String,         // 代售平台商品的主播ID
    pub admin_id: String,        // 发布自营商品的管理员id
    pub shareuid: String,        // 分享商品的用户ID
    #[serde(with = "rust_decimal::serde::str")]
    pub share_income: Decimal, // 分享佣金
    pub add_time: i32,           // 添加时间（机器）
    pub upd_time: i32,           // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
