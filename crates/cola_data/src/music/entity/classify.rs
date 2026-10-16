// entity/classify.rs
// 数据中心 - MUSIC - entity - 类别
// 2026/8/3 16:35 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 音乐 类别表
/// * `pg schema`: `cola_music` - PG 模式
/// * `table name`: `classify` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicClassifyEntity {
    pub id: i64,                            // ID
    pub _id: Option<String>,                // UUID v4
    pub name: String,                       // 分类名称
    pub name_en: String,                    // 分类英文名称
    pub description: Option<String>,        // 描述
    pub cover_url: Option<String>,          // 封面url
    pub file_url: Option<String>,           // 音频文件url
    pub tags: Option<String>,               // 标签（逗号分隔或 JSON 字符串）
    pub original: Option<bool>,             // 是否为原创音乐
    pub views: Option<i64>,                 // 浏览/播放量
    pub likes: Option<i64>,                 // 点赞量
    pub steps: Option<i64>,                 // 被踩数量
    pub collects: Option<i64>,              // 收藏量
    pub comments: Option<i64>,              // 评论数量
    pub shares: Option<i64>,                // 分享数量
    pub is_public: Option<bool>,            // 是否公开
    pub status: Option<i16>,                // 状态码:（0-草稿, 1-正常, 2-审核中, 3-已下架等）
    pub is_deleted: Option<bool>,           // 逻辑删除
    pub sort: Option<i16>,                  // 排序: 默认9999
    pub add_time: Option<i64>,              // 创建时间（旧版时间戳）
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,  // 删除时间（软删除）
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
/// * `描述`: 给SQLX使用
pub const MUSIC_CLASSIFY_COLUMNS: &str = r#"
    id, uuid, author, author_name, title, description, cover_url, file_url,
    href, length, duration, size, format, bitrate, category_id, category_name,
    tags, lyric_url, source, original, copyright, views, likes, steps,
    collects, comments, shares, is_public, status, use_nums,
    add_time, created_at, updated_at, deleted_at, reviewed_at
"#;

//////// END
