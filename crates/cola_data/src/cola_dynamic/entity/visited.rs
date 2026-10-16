// data/src/cola_dynamic/entity/visited.rs
// 🗄 数据 - ⏹ 可乐动态 - Entity - 浏览记录表
// 2026/6/19 16:47

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 动态 - 浏览记录 表
/// * `pg schema`: `cola_dynamic` - PG 模式
/// * `table name`: `visited` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DynamicVisitedEntity {
    pub id: i64,                           // 动态ID(自增)
    pub _id: i64,                          // 雪花ID(可选)
    pub uid: i64,                          // 用户ID
    pub dynamic_id: i64,                   // 动态ID
    pub remark: Option<String>,            // 备注(可选)
    pub status: i16,                       // 状态
    pub is_del: i16,                       // 是否删除
    pub del_time: Option<i64>,             // 删除时间(机器)
    pub add_time: i64,                     // 添加时间(机器)
    pub upt_time: i64,                     // 更新时间(机器)
    pub created_at: Option<DateTime<Utc>>, // 创建时间(人类)
    pub updated_at: Option<DateTime<Utc>>, // 更新时间(人类)
}

//////// END
