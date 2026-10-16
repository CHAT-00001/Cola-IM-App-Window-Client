// handler/ban_reason.rs  -- 实体层 封禁原因
// 2026/6/13 08:07

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播流主播封禁原因 实体
/// * `table_name` live_stream_anchor_ban_reason
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamAnchorBanReasonEntity {
    pub id: i64,                   // ID
    pub operator_uid: i64,         // 操作者ID
    pub icon: String,              // 图标
    pub name: String,              // 中文名称
    pub name_en: String,           // 英文名称
    pub description: String,       // 中文描述
    pub description_en: String,    // 英文描述
    pub remark: String,            // 签注(默认为空)
    pub status: i16,               // 状态: 0. 禁用 1. 启用 (默认生效)
    pub add_time: i32,             // 添加时间 - 机器
    pub upd_time: i32,             // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

//////// END
