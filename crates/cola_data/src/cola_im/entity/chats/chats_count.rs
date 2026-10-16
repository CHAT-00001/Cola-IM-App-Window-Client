// cola_data/src/cola_im/entity/chats/chats_count.rs
// 🗄 数据 - ⏹ 可乐IM - entity - chats - 计数
// 2026/8/3 15:30 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 聊天会话 - 计数
/// * `pg shcema`: `cola_im`
/// * `table name`: `chats_count`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImChatsCountEntity {
    pub chat_id: i64, // 聊天会话 ID
    // -- 主动
    pub by_post_count: i32,    // 我的视频数量
    pub by_view_count: i32,    // 我看过的视频数量
    pub by_like_count: i32,    // 我点赞的视频数量
    pub by_collect_count: i32, // 我收藏的视频数量
    pub by_share_count: i32,   // 我分享的数量
    pub by_report_count: i32,  // 我举报的数量
    // -- 被动
    pub at_likes_count: i32,    // 被点赞数量
    pub at_collects_count: i32, // 被收藏数量
    pub at_view_count: i32,     // 被浏览数量

    pub level: Option<i16>,        // 用户等级
    pub author_level: Option<i16>, // 主播等级
    // -- 来源 --
    pub lat: Option<f64>,              // 纬度
    pub lng: Option<f64>,              // 经度
    pub login_ip: std::net::IpAddr,    // 当前登录IP
    pub register_ip: std::net::IpAddr, // 注册IP
    // -- 状态 --
    pub status: Option<i16>, // 状态
    // -- time --
    pub add_time: i64,                     // 创建时间（兼容PHP程序）
    pub upd_time: Option<i64>,             // 同步时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const IM_CHATS_SETTING_COLUMNS: &str = r#"
    id, uid, icon, name, remark, is_deleted, status,
    addtime, updtime, created_at, updated_at, deleted_at
"#;

//////// END
