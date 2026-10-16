// data/src/cola_dynamic/entity/collect.rs
// 🗄 数据 - ⏹ 可乐动态 - entity - 收藏表
// 2026/6/19 16:44

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 动态 - 收藏 表
/// * `pg schema`: `cola_dynamic` - PG 模式
/// * `table name`: `collect` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DynamicCollectEntity {
    pub id: i64,                           // 动态 ID(自增)
    pub _id: i64,                          // 雪花 ID(可选)
    pub uid: i64,                          // 用户 ID
    pub dynamic_id: i64,                   // 动态 ID
    pub remark: Option<String>,            // 备注(可选)
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 是否删除(新版)
    pub is_del: i16,                       // 是否删除
    pub add_time: i64,                     // 添加时间(机器)
    pub upt_time: i64,                     // 更新时间(机器)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

//////// END
