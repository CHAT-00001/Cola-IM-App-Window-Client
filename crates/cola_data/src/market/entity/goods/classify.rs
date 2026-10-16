// market/handler/goods/classify.rs
// 数据中心 - MARKET - entity - 商品 - 分类表
// 2026/6/18 10:59

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品 - 分类表
/// * `pg schema`: `market`
/// * `table name`: `shop_classify`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopClassifyEntity {
    pub id: Option<i64>,                  // ID (自增 / 雪花)
    pub gc_id: i64,                       // id
    pub gc_name: i64,                     // 中文名称
    pub gc_name_en: String,               // 英文名称
    pub gc_parentid: i16,                 // 父级分类ID
    pub gc_one_id: i16,                   // 一级分类ID
    pub gc_sort: i16,                     // 老版排序
    pub gc_isshow: i16,                   // 是否展示 0 否 1 是
    pub sort: i16,                        // 排序 默认9999
    pub gc_grade: String,                 // 等级
    pub gc_icon: String,                  // 图标
    pub gc_isvirtual: i16,                // 是否虚拟:
    pub visibility: i32,                  // 可见度
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}

//////// END
