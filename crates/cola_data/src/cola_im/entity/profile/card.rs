// cola_data/src/cola_im/entity/profile/card.rs
// 🗄 数据 - ⏹ 可乐IM - entity - profile - 名片
// 2026-01-16 09:37:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 联系人 名片 实体表
/// * `pg schema`: `cola_im`
/// * `table name`: `card`
/// * `类型`: 1. 文字 2. 语音 3. 照片 4. 视频 5. 位置 7. 表情包 8. 红包 9. 转账 ...
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ContactCardEntity {
    pub id: i64,                           // id
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub contact_type: i16,                 // 联系人类型
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 被踩数量
    pub contacts: i32,                     // 通信录数量
    pub first_name: String,                // 第一名称(姓)
    pub last_name: String,                 // 最后名称(名字)
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间（机器）
    pub upd_time: i64,                     // 更新时间（机器）
    pub created_at: Option<DateTime<Utc>>, // 创建时间(人类)
    pub updated_at: Option<DateTime<Utc>>, // 更新时间(人类)
    pub send_time: i64,
    pub sync_time: i64,
}

//////// END
