// cola_data/src/cola_live/entity/kick.rs  -- 数据中心 - LIVE - entity - 踢人
// 2026/6/13 07:06

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 直播间踢人表
/// * `pg schema`: `cola_live` - PG 模式
/// * `table_name`: `room_kick` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveRoomKickEntity {
    pub id: i64,                   // ID
    pub operator_uid: i64,         // 操作者ID
    pub live_id: i64,              // 直播间ID
    pub record_id: i64,            // 直播场次记录ID
    pub user_id: i64,              // 目标用户ID
    pub duration: i64,             // 时长
    pub status: i16,               // 状态: 0. 失效 1. 有效(默认)
    pub add_time: i64,             // 添加时间 - 机器
    pub upd_time: i64,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建于 - 人类
    pub updated_at: DateTime<Utc>, // 更新于 - 人类
}
////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const LIVE_ROOM_KICK_COLUMNS: &str = r#"
    id, operator_uid, live_id, record_id, user_id, duration,
    status,
    add_time, upd_time, created_at, updated_at
"#;

//////// END
