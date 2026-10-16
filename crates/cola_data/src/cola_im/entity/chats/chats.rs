// cola_data/src/cola_im/entity/chats/chats_session.rs
// 🗄 数据 - ⏹ 可乐IM - entity - chats - 聊天 会话表
// 2026/8/3 15:35 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 聊天会话 - 会话
/// * `pg schema`: `cola_im`
/// * `table name`: `chats_session`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImChatsSessionEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 操作者用户ID
    pub chat_type: i16,                    // 会话类型: 1单聊 2群聊 3系统 4机器人
    pub target_id: i64,                    // 目标ID: 用户ID/群ID
    pub icon: Option<String>,              // 图标
    pub name: Option<String>,              // 英文名称
    pub name_zh: Option<String>,           // 中文名称
    pub remark: Option<String>,            // 备注
    pub last_msg_id: Option<i64>,          // 最后一条消息ID
    pub last_msg_preview: Option<String>,  // 最后一条消息摘要
    pub last_msg_time: i64,                // 最后一条消息时间（机器）
    pub unread_count: i32,                 // 未读数量
    pub is_pin: bool,                      // 是否置顶
    pub is_started: bool,                  // 是否星标: 默认false
    pub is_mute: bool,                     // 是否免打扰: 默认false
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub status: i16,                       // 状态码: 0无效 1有效
    pub add_time: i64,                     // 添加时间（机器）
    pub upd_time: i64,                     // 更新时间（机器）
    pub created_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub updated_at: Option<DateTime<Utc>>, // 更新时间（人类）
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（人类）
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const IM_CHATS_SESSION_COLUMNS: &str = r#"
    id, uid, chat_type, target_id,
    icon, name, name_zh, remark,
    last_msg_id, last_msg_preview, last_msg_time,
    unread_count,
    is_pin, is_started, is_mute, is_deleted,
    status,
    add_time, upd_time,
    created_at, updated_at, deleted_at
"#;

//////// END
