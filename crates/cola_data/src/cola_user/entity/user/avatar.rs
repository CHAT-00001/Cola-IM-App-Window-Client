// cola_data/src/user/entity/user/avatar.rs -- DATA - USER - entity - 用户 - 头像表
// 2026/3/30 05:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户中心 - 用户头像表
/// * `pg schema`: `cola.user` -- PG 模式
/// * `table name`: `user_avatar`  -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserAvatarEntity {
    pub id: i64,                              // 用户 ID
    pub _id: Option<String>,                  // UUID v4
    pub user_id: i64,                         // 用户 ID
    pub avatar: Option<String>,               // 头像
    pub avatar_thumb: Option<String>,         // 小头像
    pub bg_img: Option<String>,               // 背景图
    pub views: i64,                           // 被浏览量
    pub likes: i64,                           // 被点赞量
    pub status: Option<i16>,                  // 状态码
    pub is_deleted: Option<bool>,             // 逻辑删除
    pub is_public: Option<bool>,              // 是否公开
    pub created_at: Option<DateTime<Utc>>,    // 创建时间
    pub updated_at: Option<DateTime<Utc>>,    // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,    // 删除时间
    pub last_pick_at: Option<DateTime<Utc>>,  // 最后选择时间
}

////////

/// # 2.[COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const USER__AVATAR_COLUMNS: &str = r#"
    id, _id, user_id, avatar, avatar_thumb, bg_img, views, likes,
    status, is_deleted, is_public, created_at, updated_at, deleted_at, last_pick_at
"#;

//////// END