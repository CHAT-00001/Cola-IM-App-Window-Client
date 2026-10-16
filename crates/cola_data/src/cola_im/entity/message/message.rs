// cola_data/src/cola_im/entity/message.rs
// 🗄 数据 - ⏹ 可乐IM - entity - message - 消息表
// 2026-07-07 14:00

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - IM 消息 消息表
/// * `pg schema`: `cola_im`
/// * `table name`: `message`
/// * `status`: 0. 草稿 1. 正在发送 2. 服务器同步 3. 发送成功 4. 下发消息 5. 接收 6. 已读 7. 已读回执
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImMessageEntity {
    pub id: i64,                           // 消息 ID
    pub send_id: Option<String>,           // 发送 ID
    pub user_id: i64,                      // 用户 ID
    pub at_id: i64,                        // 父消息 ID (引用上下文时)
    pub channel_id: i16,                   // 通道
    pub content: String,                   // // 内容
    pub likes: i32,                        // 点赞数量
    pub dislikes: i32,                     // 不喜欢数量
    pub visibility: i16,                   // 可见性
    pub color: Option<String>,             // 颜色（默认 #FFFFFF）
    pub mode: i16,                         // 模式: 0. 固定 1. 滚动
    pub playload: i32,                     // 消息负载
    pub duration: i16,                     // 弹幕留存时间（固定字幕时生效）
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 是否删除
    pub send_time: i64,                    // 发送时间（客户端生成）
    pub sync_time: i64,                    // 服务器同步时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
pub const IM_MESSAGE_COLUMNS: &str = r#"
    id, user_id, classify_id, name, name_en, is_banned, is_active,
    url, sort, status,
    add_time, upd_time
"#;

//////// END
