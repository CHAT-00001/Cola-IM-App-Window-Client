// cola_data/src/music/entity/music/music
// 数据中心 - MUSIC - entity - 音乐 - 音乐表
// 2026/5/19 21:12

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 音乐 音乐表
/// * `pg schema`: `cola_music` - PG 模式
/// * `table name`: `music` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicEntity {
    pub id: i64,                            // id
    pub uuid: Option<String>,               // uuid
    pub author: Option<i64>,                // 用户id（上传者/创作者）
    pub author_name: Option<String>,        // 歌手/作者名称
    pub title: Option<String>,              // 标题
    pub description: Option<String>,        // 描述
    pub cover_url: Option<String>,          // 封面url
    pub file_url: Option<String>,           // 音频文件url
    pub href: String,                       // 视频/源链接url
    pub length: Option<String>,             // 时长（旧版文本格式，如 "03:45"）
    pub duration: Option<i32>,              // 时长(ms，毫秒)
    pub size: Option<i64>,                  // 文件大小（字节 bytes）
    pub format: Option<String>,             // 音频格式（如 mp3, flac, wav, m4a）
    pub bitrate: Option<i32>,               // 比特率/音质（如 320000 代表 320kbps）
    pub classify_id: Option<i64>,           // 分类ID
    pub tags: Option<String>,               // 标签（逗号分隔或 JSON 字符串）
    pub lyric_url: Option<String>,          // 歌词文件url (lrc 等)
    pub source: Option<String>,             // 音乐来源（如 internal, douyin, qq_music, netease）
    pub original: Option<bool>,             // 是否为原创音乐
    pub copyright: Option<String>,          // 版权信息
    pub views: Option<i64>,                 // 浏览/播放量
    pub likes: Option<i64>,                 // 点赞量
    pub steps: Option<i64>,                 // 被踩数量
    pub collects: Option<i64>,              // 收藏量
    pub comments: Option<i64>,              // 评论数量
    pub shares: Option<i64>,                // 分享数量
    pub is_public: Option<bool>,            // 是否公开
    pub status: Option<i16>,                // 状态（0-草稿, 1-正常, 2-审核中, 3-已下架等）
    pub use_nums: Option<i64>,              // 被使用数量（拍同款/做BGM次数）
    pub add_time: Option<i32>,              // 创建时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,  // 删除时间（软删除）
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
pub const MUSIC_COLUMNS: &str = r#"
    id, uuid, author, author_name, title, description, cover_url, file_url,
    href, length, duration, size, format, bitrate, classify_id,
    tags, lyric_url, source, original, copyright, views, likes, steps,
    collects, comments, shares, is_public, status, use_nums,
    add_time, created_at, updated_at, deleted_at, reviewed_at
"#;

//////// END
