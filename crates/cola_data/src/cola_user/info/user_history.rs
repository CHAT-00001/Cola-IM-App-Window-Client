// cola_data/src/cola_user/info/user_history.rs -- 用户头像与昵称历史信息
// 2026/9/23 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户资料历史信息
/// * `desc`: `统一承载头像和昵称历史记录`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserHistoryInfo {
    pub id: i64,                   // 历史记录 ID
    pub uid: i64,                  // 所属用户 ID
    pub value: String,             // 头像路径或昵称
    pub status: i16,               // 1有效 0逻辑删除
    pub created_at: DateTime<Utc>, // 创建时间
}

////////

impl From<crate::cola_user::entity::user_history::UserHistoryEntity> for UserHistoryInfo {
    fn from(entity: crate::cola_user::entity::user_history::UserHistoryEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            value: entity.value,
            status: entity.status,
            created_at: entity.created_at,
        }
    }
}

//////// END
