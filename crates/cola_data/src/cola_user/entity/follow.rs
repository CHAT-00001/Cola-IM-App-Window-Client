// cola_data/src/user/entity/follow.rs -- 数据 - USER - entity - 关注记录表
// 2026/6/18 10:04 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户中心 - 关注记录表
/// * `pg schema`: `cola_user`
/// * `table name`: `follow`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserFollowEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 操作者用户ID
    pub user_id: i64,                      // 目标用户ID
    pub name: Option<String>,              // 关注时的名字
    pub remark: Option<String>,            // 备注
    pub is_started: bool,                  // 是否星标: 默认false
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub status: i16,                       // 状态码: 0无效 1有效
    pub add_time: i32,                     // 添加时间（PHP旧版）
    pub upd_time: i32,                     // 更新时间（PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub updated_at: Option<DateTime<Utc>>, // 更新时间（人类）
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（人类）
}

////////

/// # [COLUMNS] - sqlx数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_FOLLOW_COLUMNS: &str = r#"
    id, uid, user_id, name, remark,
    is_started, is_deleted, status,
    add_time, upd_time, created_at, updated_at, deleted_at
"#;

//////// END
