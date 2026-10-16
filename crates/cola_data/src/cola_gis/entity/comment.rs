// cola_data/src/cola_gis/entity/add  -- 可乐GIS - entity -  评论
// 2026-01-16 09:37:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - POI 评论
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct PoiCommentEntity {
    pub id: i64,                           // id
    pub user_id: i64,                      // 用户 ID
    pub poi_id: i64,                       // POI ID
    pub parent_id: Option<i64>,            // 父级ID
    pub comment_type: i16,                 // 评论类型
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub likes: i32,                        // 点赞数量
    pub dislikes: i32,                     // 讨厌数量
    pub collects: i32,                     // 收藏数量
    pub reply: i32,                        // 回复数量
    pub visibility: i16,                   // 可见性
    pub region_code: Option<String>,       // i18n
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
    pub send_time: i64,
    pub sync_time: i64,
}

////////

/// # [COLUMNS] - 数据表字段
pub const POI_COMMENT_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, gis_id, parent_id, content, at_uids,
    thumb_url, photos_url, video_url, voice_url, lat, lng,
    likes, steps, collects, reply, visibility, region_code,
    status, deleted_at, deleted_by, addtime, created_at, updated_at
"#;

//////// END
