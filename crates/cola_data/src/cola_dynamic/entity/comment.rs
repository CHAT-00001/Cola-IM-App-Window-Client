// data/src/cola_dynamic/entity/file.rs
// 🗄 数据 - ⏹ 可乐动态 -  entity - 评论表
// 2026/6/19 16:22

////////
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

////////

/// # [ENTITY] - 动态 评论表
/// * `pg schema`: `cola_dynamic` - PG 模式
/// * `table name`: `comments` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct DynamicCommentEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 用户ID
    pub dynamic_id: i64,                   // 动态 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub comment_type: i16,                 // 类型 (旧版)
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub media: Option<Vec<i64>>,           // 媒体IDs (新版)
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 不喜欢数量
    pub collects: i32,                     // 收藏数量
    pub reply: i32,                        // 回复数量（是父评论时）
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 是否删除 (新版)
    pub add_time: i64,                     // 添加时间(兼容旧版)
    pub upd_time: Option<i64>,             // 更新时间(兼容旧版)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

//////// END
