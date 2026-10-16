// cola_data/src/music/info/user.rs
// 数据 - MUSIC - info - 用户资料
// 2026/8/24 12:33 Created.

////////

use crate::music::entity::album::MusicAlbumEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 音乐用户资料安全视图/DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicUserInfo {
    pub id: i64,
    pub uuid: String,
    pub uid: i64,
    pub name: String,
    pub name_zh: String,
    pub description: String,
    pub cover_url: String,
    pub music_qty: i32,
    pub category_id: i64,
    pub tags: Vec<String>, // 转为更友好的 Vec<String> 方便前端使用
    pub source: String,
    pub copyright: String,
    pub views: i64,
    pub likes: i64,
    pub collects: i64,
    pub shares: i64,
    pub review_reason: String,
    pub status: i16,
    pub is_public: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl MusicUserInfo {
    /// 1. 兜底/空对象函数 (Empty)
    pub fn empty() -> Self {
        Self {
            id: 0,
            uuid: String::new(),
            uid: 0,
            name: String::new(),
            name_zh: String::new(),
            description: String::new(),
            cover_url: String::new(),
            music_qty: 0,
            category_id: 0,
            tags: Vec::new(),
            source: String::new(),
            copyright: String::new(),
            views: 0,
            likes: 0,
            collects: 0,
            shares: 0,
            review_reason: String::new(),
            status: 0,
            is_public: false,
            created_at: None,
            updated_at: None,
        }
    }
}

/// 2. 从实体类转换的实现 (From)
impl From<MusicAlbumEntity> for MusicUserInfo {
    fn from(entity: MusicAlbumEntity) -> Self {
        // 安全解析 tags 字段（假设数据库中是以逗号分隔的字符串，如 "pop,rock"）
        let tags = entity
            .tags
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Self {
            id: entity.id,
            uuid: entity.uuid.unwrap_or_default(),
            uid: entity.uid,
            name: entity.name.unwrap_or_default(),
            name_zh: entity.name_zh.unwrap_or_default(),
            description: entity.description.unwrap_or_default(),
            cover_url: entity.cover_url.unwrap_or_default(),
            music_qty: entity.music_qty.unwrap_or_default(),
            category_id: entity.category_id.unwrap_or_default(),
            tags,
            source: entity.source.unwrap_or_default(),
            copyright: entity.copyright.unwrap_or_default(),
            views: entity.views,
            likes: entity.likes,
            collects: entity.collects,
            shares: entity.shares,
            review_reason: entity.review_reason.unwrap_or_default(),
            status: entity.status,
            is_public: entity.is_public.unwrap_or_default(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//////// END
