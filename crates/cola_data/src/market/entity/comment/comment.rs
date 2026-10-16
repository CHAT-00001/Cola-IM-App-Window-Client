// data/src/market/entity/hotlist
// 数据 - MARKET - entity - 商品评论表
// 2026/8/14 09:22 Created.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 商品 评论表
/// * `pg schema`: `cola_market` - PG 模式
/// * `table name`: `comments` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsCommentEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 作者 ID
    pub goods_id: i64,                     // 商品 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub comment_type: i16,                 // 类型
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub media_ids: Option<Vec<i64>>,       // 媒体对象 (新版本)
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 不喜欢数量
    pub collects: i32,                     // 收藏量
    pub reply: i32,                        // 回复数量（是父评论时）
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub is_pinned: Option<bool>,           // 是否置顶
    pub is_deleted: Option<bool>,          // 是否删除
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const GOODS_COMMENT_COLUMNS: &str = r#"
    id, _id, uid, goods_id,
    parent_id, comment_type,
    content, photos_url,video_url, voice_url, duration, media_ids,
    likes, dislikes, collects,reply, visibility, region_code,
    status, is_pinned, is_deleted,
    add_time, upd_time,created_at, updated_at, deleted_at
"#;

//////// END
