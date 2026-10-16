// cola-data/src/cola_live/entity/class.rs  -- 数据中心 - LIVE - entity - 分类
// 2026/6/13 07:06

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 分类表
/// * `pg schema`: `cola_live`
/// * `table_name`: `cola_live.live_class`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveClassEntity {
    pub id: i64,                   // ID
    pub uid: i64,                  // 操作者ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub icon: String,              // 图标
    pub action_uid: i64,           // 最后操作者ID
    pub sort: i16,                 // 排序(默认9999)
    pub is_hot: i16,               // 是否热门(默认0)
    pub is_recommended: i16,       // 是否推荐(默认0)
    pub status: i16,               // 状态: 0. 禁用 1. 启用(默认)
    pub add_time: i64,             // 添加时间 - 机器
    pub upd_time: i64,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

//////// END
