// market/entity/goods/collect.rs
// 数据中心 - MARKET - entity - 商品 - 收藏表
// 2026/8/3 23:08 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品 - 收藏表
/// * `pg schema`: `market`
/// * `table name`: `goods_collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsCollectEntity {
    pub id: i64,                          // ID (自增 / 雪花)
    pub uuid: Option<String>,             // uuid v4
    pub uid: i64,                         // 操作者用户ID
    pub goods_id: i64,                    // 目标商品ID
    pub goods_uid: i64,                   // 目标商品持有人ID
    pub status: i16,                      // 状态
    pub remark: Option<String>,           // 备注
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}

//////// END
