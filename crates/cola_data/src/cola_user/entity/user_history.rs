// cola_data/src/cola_user/entity/user_history.rs -- 用户头像与昵称历史实体
// 2026/9/23 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户资料历史
/// * `desc`: `头像、昵称历史表共用实体`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserHistoryEntity {
    pub id: i64,                   // 历史记录 ID
    pub uid: i64,                  // 所属用户 ID
    pub value: String,             // 头像路径或昵称
    pub status: i16,               // 1有效 0逻辑删除
    pub created_at: DateTime<Utc>, // 创建时间
}

//////// END
