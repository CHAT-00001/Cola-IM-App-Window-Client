// cola_data/src/cola_im/entity/message_media.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 消息媒体
// 2026-07-07 15:15

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 消息 - 媒体表
/// * `pg schema`: `cola_im`
/// * `table name`: `message_meida`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageMediaEntity {
    pub id: i64,                 // 用户ID
    pub name: String,            // 名称
    pub name_zh: String,         // 中文名称
    pub icon: String,            // 媒体图标
    pub media_type: String,      // 媒体类型
    pub url: String,             // 媒体URL
    pub size: i64,               // 体积
    pub status: i16,             // 状态
    pub uploaded_at: i64,        // 上传时间 - 机器
    pub deleted_at: Option<i64>, // 删除时间 - 机器
}

//////// END
