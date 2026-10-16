// cola_data/src/live/entity/room/guard.rs  - 直播间 守护表
// 2026/6/13 07:07

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 守护表
/// * `pg schema`: `cola_live`
/// * `table_name`: `room_guard`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamGuardEntity {
    pub id: i64,        // ID
    pub uid: i64,       // 操作者ID
    pub record_id: i64, // 直播场次记录ID
    pub user_id: i64,   // 目标用户ID
}

//////// END
