// data/src/live/entity/room/shut.rs
// 数据 - LIVE - entity - 房间 - 禁言表
// 2026/8/20 17:30 Created.

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播间 闭嘴表
/// * `pg schema`: `cola_live` - PG 模式
/// * `table_name` `room_shut` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveRoomShutEntity {
    pub id: i64,          // ID
    pub uid: i64,         // 操作者 ID
    pub room_id: i64,     // 直播间 ID
    pub user_id: i64,     // 目标用户 ID
    pub is_deleted: bool, // 是否删除
    pub status: i16,      // 状态码: 0. 失效 1. 有效
    pub add_time: i64,    // 添加时间
    pub end_time: i64,    // 结束时间
}
//////// END
