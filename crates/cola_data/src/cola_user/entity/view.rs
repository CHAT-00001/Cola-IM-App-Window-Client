// cola_data/src/user/entity/view.rs -- 数据中心 - 可乐用户 - entity - 浏览记录表
// 2026/8/7 15:59 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 浏览记录表
/// * `pg schema`: `cola.user` -- PG 模式
/// * `table name`: `view` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserViewEntity {
    pub id: i64, // ID (自增 / 雪花)
    #[serde(rename = "_id")] // 重命名
    pub _id: Option<String>, // UUID v4
    pub uid: i64, // 用户ID
    pub video_id: i64, // 目标视频ID
    pub time: Option<i32>, // 观看时间（秒）
    pub count: Option<i32>, // 观看次数: 最小1（防溢出改用 u32）
    pub status: i16, // 状态码: 0无效 1有效
    pub is_deleted: bool, // 是否删除: 默认false
    pub add_time: i64, // 添加时间（兼容旧版PHP，单位：秒）
    pub upd_time: i64, // 更新时间（兼容旧版PHP，单位：秒）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（软删除）
}

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_VIEW_COLUMNS: &str = r#"
    id, _id, uid, video_id, time, count, status, is_deleted,
    add_time, upd_time, created_at, updated_at, deleted_at
"#;

//////// END
