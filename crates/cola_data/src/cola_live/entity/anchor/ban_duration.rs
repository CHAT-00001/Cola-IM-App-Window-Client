// handler/ban_duration.rs  -- handler 直播 封禁时长
// 2026/6/13 08:19

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播封禁时长 实体
/// * `table_name` live_stream_ban_duration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamBanDurationEntity {
    pub id: i32,                   // ID
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub duration: i64,             // 时长(秒)
    pub is_admin: i16,             // 是否管理员专属(默认0, 超过180d的需要管理员才可选)
    pub sort: i16,                 // 排序(默认9999)
    pub status: i16,               // 状态: 0. 失效 1. 生效 (默认生效)
    pub add_time: i32,             // 添加时间 - 机器
    pub upd_time: i32,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

//////// END
