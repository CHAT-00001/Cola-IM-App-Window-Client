// cola_data/src/music/entity/user.rs
// 数据中心 - MUSIC - entity - 用户表
// 2026/8/24 Updated.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

////////

/// # [ENTITY] - 音乐用户表
/// * `pg schema`: `cola_music` - PG 模式
/// * `table name`: `user` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicUserEntity {
    pub id: i64,                           // 关联用户ID
    pub _id: Option<String>,               // UUID v4 (通常为日志id)
    pub name: Option<String>,              // 名称
    pub name_en: Option<String>,           // 英文名称
    pub is_artist: Option<bool>,           // 是否创作者
    pub play_count: i64,                   // 播放数量
    pub music_count: i64,                  // 发布的音乐数量
    pub collect_count: i64,                // 收藏的音乐数量
    pub like_count: i64,                   // 点赞的音乐数量
    pub sort_order: i32,                   // 排序索引/权重（客户端拖拽排序后更新此值）
    pub author_name: Option<String>,       // 歌手/作者名称
    pub title: Option<String>,             // 标题
    pub cover_url: Option<String>,         // 封面url
    pub file_url: Option<String>,          // 音频文件url
    pub duration: Option<i32>,             // 时长(ms，毫秒)
    pub format: Option<String>,            // 音频格式
    pub status: i16,                       // 状态码: 0失效 1有效 2禁用 3审核中 4驳回
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
pub const MUSIC_USER_COLUMNS: &str = r#"
    id, uuid, playlist_id, music_id, sort_order,
    author_name, title, cover_url, file_url, duration, format,
    added_by, created_at, updated_at, deleted_at
"#;

//////// END
