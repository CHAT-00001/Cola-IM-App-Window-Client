// cola_data/src/cola_im/entity/contact_relation.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 联系人 关系
// 2026/7/7 16:02

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// #  [ENTITY] - 联系人 - 关系 实体表结构
/// * `pg schema`: `cola_im`
/// * `table name`: `contact_relation`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ContactRelationEntity {
    pub id: i64,                           // 关系 ID
    pub name: String,                      // 名称
    pub name_zh: String,                   // 中文名称
    pub icon: Option<String>,              // 关系图标
    pub sort: Option<i64>,                 // 排序
    pub status: i16,                       // 状态码
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
}

//////// END
