// data/src/market/entity/goods/goods.rs
// 数据 - MARKET - entity - 商品表
// 2026/8/3 22:45 Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品表
/// * `pg schema`: `market` - PG 模式
/// * `table name`: `goods` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsEntity {
    pub id: i64,                  // ID ( 自增 / 雪花 )
    pub _id: Option<String>,      // UUID v4
    pub uid: i64,                 // 操作者用户ID
    pub city_id: Option<i64>,     // 城市 ID
    pub name: String,             // 名称
    pub name_en: String,          // 英文名称
    pub name_tag: Option<String>, // 多语言JSONB
    pub no: String,               // 货号
    pub one_classid: i64,         // 一级分类
    pub two_classid: i64,         // 二级分类
    pub three_classid: i64,       // 三级分类
    pub video_url: String,        // 视频地址
    pub video_thumb: String,      // 视频封面
    pub video_length: i32,        // 视频长度
    pub thumbs: String,           // 封面
    pub content: String,          // 内容
    pub pictures: String,         // 内容图集
    pub specs: String,            // 商品规格
    pub postage: i32,             // 邮费
    pub hits: i32,                // 点击数
    pub isrecom: i16,             // 推荐，0否1是(旧版)
    pub sale_nums: i32,           // 总销量
    pub refuse_reason: String,    // 商品拒绝原因
    pub issale: i16,              // 商品是否在直播间销售 0 否 1 是(针对用户自己发布的商品)
    #[sqlx(rename = "type")]
    pub r#type: i16, // 类型: 0. 站内 1. 站外 2. 自营 3. 虚拟
    pub price: Option<Decimal>,   // 价格 (参考, 成交价看可选项的价格)
    pub original_price: Option<Decimal>, // 站外商品原价
    pub present_price: Option<Decimal>, // 站外商品现价
    pub goods_desc: Option<String>, // 站外商品简介
    pub href: Option<String>,     // 站外商品链接
    pub live_isshow: Option<Decimal>, // 直播间是否展示商品简介 0 否 1 是 默认0
    pub low_price: Option<Decimal>, // 站外商品最低价
    pub admin_id: Option<String>, // 发布自营商品的管理员id
    pub commission: Option<String>, // 主播代卖平台商品的佣金
    pub share_income: Option<String>, // 分享给其他用户购买后获得的佣金
    pub lat: Option<f64>,         // 纬度
    pub lng: Option<f64>,         // 经度
    pub city: Option<String>,     // 城市
    pub address: Option<String>,  // 地址
    pub label_id: i32,            // 标签ID
    pub collects: i32,            // 被收藏数量
    pub shares: i32,              // 被分享数量
    pub status: i16,              // 状态码
    pub is_deleted: Option<bool>, // 是否逻辑删除
    pub is_recommend: Option<bool>, // 是否推荐
    pub add_time: i32,            // 添加时间（兼容旧版）
    pub upd_time: i32,            // 更新时间（兼容旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（逻辑删除）
}

////////

/// # [COLUMNS] - 商品表全部字段
/// * `desc`: `SQLx 映射`
pub const GOODS_COLUMNS: &'static str = r#"
        id, _id, uid, city_id, name, name_en, name_tag, no,
        one_classid, two_classid, three_classid,
        video_url, video_thumb, video_length, thumbs, content, pictures,
        specs, postage, hits, isrecom, sale_nums, refuse_reason, issale, type,
        price, original_price, present_price, goods_desc, href, live_isshow,
        low_price, admin_id, commission, share_income,
        lat, lng, city, address, label_id,
        collects, shares, status, is_deleted, is_recommend,
        add_time, upd_time, created_at, updated_at, deleted_at,
    "#;

//////// END
