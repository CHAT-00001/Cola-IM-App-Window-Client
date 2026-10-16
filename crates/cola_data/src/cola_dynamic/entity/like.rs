// data/src/cola_dynamic/entity/like.rs
// 🗄 数据 - ⏹ 可乐动态 -  Entity - 点赞表
// 2026/6/19 16:41

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 动态 - 点赞表
/// * `pg schema`: `cola_dynamic` - PG 模式
/// * `table name`: `like` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DynamicLikeEntity {
    pub id: i64,                           // ID(自增 / 雪花)
    pub _id: i64,                          // UUID v4
    pub uid: i64,                          // 用户 ID
    pub dynamic_id: i64,                   // 动态 ID
    pub remark: Option<String>,            // 备注(可选)
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 是否删除(新版)
    pub is_del: i16,                       // 是否删除(旧版)
    pub add_time: i64,                     // 添加时间(兼容旧版PHP)
    pub upt_time: i64,                     // 更新时间(兼容旧版PHP)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

//////// END
