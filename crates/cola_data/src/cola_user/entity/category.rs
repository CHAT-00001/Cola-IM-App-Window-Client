// cola_data/src/user/entity/category.rs -- 数据 - USER - entity - 分类表
// 2026/8/4 02:00 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 分类表
/// * `pg schema`: `cola_user`
/// * `table name`: `category`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserCategoryEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub uid: i64,                          // 所属用户ID
    pub name: String,                      // 分类名称
    pub sort: i32,                         // 排序权重
    pub description: Option<String>,       // 分类描述
    pub icon: Option<String>,              // 分类图标
    pub status: Option<i16>,               // 状态码: 0失效 1有效
    pub is_deleted: Option<i16>,           // 是否删除: 0未删除 1已删除
    pub create_time: i64,                  // 创建时间（兼容PHP时间戳）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
/// * `说明`: 给 sqlx 使用
pub const USER_CATEGORY_COLUMNS: &str = r#"
    id, uid, name, sort, description, icon,
    status, is_deleted,
    create_time, created_at, updated_at, deleted_at
"#;

//////// END
