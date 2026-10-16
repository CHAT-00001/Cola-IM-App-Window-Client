// handler/ban.rs  -- handler 直播流 封禁
// 2026/6/13 07:06

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播流主播封禁 实体
/// * `table_name` live_stream_anchor_ban
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamAnchorBanEntity {
    pub id: i64,                   // ID
    pub operator_uid: i64,         // 操作者ID
    pub record_id: Option<i64>,    // (可选,默认当前场)关联直播场次记录ID
    pub user_id: i64,              // 目标用户ID
    pub ban_type: i16,             // 封禁类型
    pub ban_reason: i16,           // 封禁原因ID
    pub remark: String,            // 签注(默认为空)
    pub status: i16,               // 状态: 0. 失效 1. 生效 (默认生效) 2. 申诉 3. 驳回 4. 锁死
    pub add_time: i32,             // 添加时间 - 机器
    pub ban_end_time: i32,         // 结束时间 - 机器
    pub created_at: DateTime<Utc>, // 创建时间 - 人类
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类
}

//////// END
