// cola_data/src/cola_im/entity/count.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 用户名片统计
// 2026-07-07 14:00

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户名片 计数 实体表
/// * `pg schema`: `cola_im`
/// * `table name`: `card_count`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct CardCountEntity {
    pub card_id: i64,                              // 用户名片 ID
    pub favorited_count: i64,                      // 被多少人特别关心
    pub stared_count: i64,                         // 被多少人星标
    pub blacked_count: i64,                        // 被多少人拉黑
    pub send_message_count: i64,                   // 发送的消息条数
    pub reply_message_count: i64,                  // 收到的消息条数
    pub request_count: i32,                        // 发出来多少次好友请求
    pub add_time: i64,                             // 添加时间 - 机器
    pub upd_time: i64,                             // 更新时间 - 机器
    pub created_at: chrono::DateTime<chrono::Utc>, // 创建时间 - 人类
    pub updated_at: chrono::DateTime<chrono::Utc>, // 更新时间 - 人类
}

//////// END
