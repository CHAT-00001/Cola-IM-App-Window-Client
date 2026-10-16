// cola_data/src/cola_im/entity/chats/chats_setting.rs
// 🗄 数据 - ⏹ 可乐IM - entity - chats - 设置表
// 2026/8/3 15:25 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 聊天会话 - 设置
/// * `pg schema`: `cola_im`
/// * `table name`: `chats_setting`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImChatsSettingEntity {
    pub id: i64,        // id
    pub uid: i64,       // 操作者用户ID
    pub chat_type: i16, // 会话类型: 1单聊 2群聊 3系统 4机器人
    pub target_id: i64, // 目标ID: 用户ID/群ID

    pub is_pin: bool,               // 是否置顶
    pub is_starred: bool,           // 是否星标
    pub is_mute: bool,              // 是否免打扰
    pub is_show_notification: bool, // 是否显示通知
    pub notify_mode: i16,           // 通知方式: 0默认 1全部 2仅@我 3关闭
    pub is_save_to_contacts: bool,  // 是否保存到通讯录
    pub wallpaper: Option<String>,  // 聊天背景
    pub ext: Option<String>,        // 扩展JSON

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
pub const IM_CHATS_SETTING_COLUMNS: &str = r#"
    id, uid, chat_type, target_id,
    is_pin, is_starred, is_mute,
    is_show_notification, notify_mode,
    is_save_to_contacts,
    wallpaper, ext,
    is_deleted, status,
    add_time, upd_time,
    created_at, updated_at, deleted_at
"#;

//////// END
