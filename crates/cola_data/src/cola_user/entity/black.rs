// cola_data/src/user/entity/black.rs -- 数据 - USER - entity - 黑名单记录表
// 2026/6/18 10:08 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户中心 - 黑名单记录表
/// * `pg schema`: `cola_user` -- 可乐用户
/// * `table name`: `black`  -- 黑名单记录表
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserBlackEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 操作者用户ID
    pub user_id: i64,                      // 目标用户ID
    pub name: Option<String>,              // 名字
    pub remark: Option<String>,            // 备注
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub status: i16,                       // 状态码: 0无效 1有效
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（软删除）
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_BLACK_COLUMNS: &str = r#"
    id, _id, uid, user_id, name, remark, is_deleted, status,
    add_time, upd_time, created_at, updated_at, deleted_at
"#;

//////// END
