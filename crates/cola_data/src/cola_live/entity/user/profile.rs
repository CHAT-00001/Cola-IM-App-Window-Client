// cola_data/src/cola_live/entity/user/profile.rs
// ✅ LIVE - 直播域用户扩展实体
// 2026/8/20 Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播域用户扩展表
/// * `pg schema`: `cola_live` - PG 模式
/// * `table name`: `user` - 表名
/// * `condition`: `user_id 复用 cola_user.user.id；等级初始化为1，经验初始化为0`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveUserProfileEntity {
    pub user_id: i64,                      // 全局用户ID
    pub level: i16,                        // 直播用户等级
    pub experience: i64,                   // 直播用户经验值
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
