// cola_data/src/music/entity/album.rs
// 数据中心 - MUSIC - entity - 专辑
// 2026/8/3 20:45 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

////////

/// # [ENTITY] - 音乐 专辑表
/// * `pg schema`: `music`
/// * `table name`: `album`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicAlbumEntity {
    pub id: i64,                            // ID (自增 / 雪花)
    pub uuid: Option<String>,               // uuid v4
    pub uid: i64,                           // 创建者用户ID
    pub name: Option<String>,               // 名称
    pub name_zh: Option<String>,            // 中文名称
    pub description: Option<String>,        // 描述
    pub cover_url: Option<String>,          // 封面url
    pub music_qty: Option<i32>,             // 包含音乐数量(默认0)
    pub category_id: Option<i64>,           // 分类ID
    pub tags: Option<String>,               // 标签（逗号分隔或 JSON 字符串）
    pub source: Option<String>,             // 来源（如 internal, douyin, qq_music, netease）
    pub copyright: Option<String>,          // 版权信息
    pub views: i64,                         // 浏览/播放量(默认0)
    pub likes: i64,                         // 点赞量(默认0)
    pub collects: i64,                      // 收藏量(默认0)
    pub shares: i64,                        // 分享数量(默认0)
    pub review_reason: Option<String>,      // 审核原因
    pub status: i16,                        // 状态（0-草稿, 1-正常, 2-审核中, 3-已下架等）
    pub is_public: Option<bool>,            // 是否公开: 默认false(私有)
    pub is_deleted: Option<bool>,           // 是否删除: 默认false
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,  // 删除时间（软删除）
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
/// * `描述`: 给SQLX使用，严格与 MusicAlbumEntity 结构体字段对齐
pub const MUSIC_ALBUM_COLUMNS: &str = r#"
    id, uuid, uid, name, name_zh, description, cover_url,
    music_qty, category_id, tags, source, copyright,
    views, likes, collects, shares, review_reason,
    status, is_public, is_deleted,
    created_at, updated_at, deleted_at, reviewed_at
"#;

//////// END
