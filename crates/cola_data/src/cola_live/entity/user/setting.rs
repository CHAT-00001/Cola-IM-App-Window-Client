// cola_data/src/cola_live/entity/user/setting.rs
// 数据 -  LIVE - entity - 用户 - 设置表
// 2026/8/20 17:23 Created.

////////

// /profile.rs
//
// 2026/8/20 Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播域 用户设置表
/// * `pg schema`: `cola_live` - PG 模式
/// * `table name`: `user_setting` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveUserSettingEntity {
    pub user_id: i64,                      // 全局用户ID
    pub live_Window: i16,                  // 直播小窗模式开关: 0关闭 1开启
    pub volume: i16,                       // 音量设置: 0-100
    pub author_level: i16,                 // 主播等级
    pub author_experience: i64,            // 主播经验值
    pub consumption: Decimal,              // 消费额
    pub open_id: Option<String>,           // 第三方标识
    pub is_zombie: Option<bool>,           // 开启僵尸粉
    pub is_super_manager: Option<bool>,    // 超级管理员
    pub status: i16,                       // 状态：0禁用 1正常
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

//////// END
