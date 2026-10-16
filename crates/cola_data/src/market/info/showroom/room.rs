// market/info/showroom/room.rs
// 市场 - info - 展厅 - 厅
// 2026/8/3 23:29 Created.

////////

use crate::market::entity::goods::goods::GoodsEntity;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

////////

/// # [INFO] - 商品 信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowRoomInfo {
    pub id: i64,
    pub uid: i64,
    pub name: String,
    pub name_en: String,
    pub no: String,
    pub one_classid: i64,
    pub two_classid: i64,
    pub three_classid: i64,
    pub thumbs: String,
    pub video_url: String,
    pub video_thumb: String,
    pub content: String,
    pub pictures: String,
    pub specs: String,
    pub original_price: String,
    pub present_price: String,
    pub commission: String,
    pub share_income: String,
    pub sale_nums: i32,
    pub is_recom: bool,
    pub r#type: i16,
    pub status: i16, // 对应原表 status 或相关逻辑字段
    pub address: String,
    pub city: String,
}

// 构造函数
impl ShowRoomInfo {
    /// # 兜底函数：返回一个空的“占位”商品，防止前端崩溃
    pub fn not_found() -> Self {
        Self {
            id: 0,
            uid: 0,
            name: "商品已下架".into(),
            name_en: "Product Unavailable".into(),
            no: "N/A".into(),
            one_classid: 0,
            two_classid: 0,
            three_classid: 0,
            thumbs: "".into(),
            video_url: "".into(),
            video_thumb: "".into(),
            content: "".into(),
            pictures: "".into(),
            specs: "".into(),
            original_price: "0.00".into(),
            present_price: "0.00".into(),
            commission: "0.00".into(),
            share_income: "0.00".into(),
            sale_nums: 0,
            is_recom: false,
            r#type: 0,
            status: 0,
            address: "未知".into(),
            city: "未知".into(),
        }
    }
}

/// # 转换实现：保留所有前端所需的字段
impl From<GoodsEntity> for ShowRoomInfo {
    fn from(e: GoodsEntity) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            name: e.name,
            name_en: e.name_en,
            no: e.no,
            one_classid: e.one_classid,
            two_classid: e.two_classid,
            three_classid: e.three_classid,
            thumbs: e.thumbs,
            video_url: e.video_url,
            video_thumb: e.video_thumb,
            content: e.content,
            pictures: e.pictures,
            specs: e.specs,
            original_price: e
                .original_price
                .unwrap_or_else(|| Decimal::from_str("0.00").unwrap())
                .to_string(),
            present_price: e
                .present_price
                .unwrap_or_else(|| Decimal::from_str("0.00").unwrap())
                .to_string(),
            commission: e.commission.unwrap_or_else(|| "0.00".to_string()),
            share_income: e.share_income.unwrap_or_else(|| "0.00".to_string()),
            sale_nums: e.sale_nums,
            is_recom: e.isrecom == 1,
            r#type: e.r#type,
            status: 0, // 假设你需要逻辑转换，这里可按需对接
            address: e.address.unwrap_or_default(),
            city: e.city.unwrap_or_default(),
        }
    }
}

//////// END
