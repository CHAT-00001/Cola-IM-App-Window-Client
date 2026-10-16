// cola_data/src/music/cmd/album.rs
// 数据中心 - MUSIC - cmd - new 新建
// 2026/8/3 21:46 Created.

////////

use crate::music::entity::album::MusicAlbumEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # [CMD] - 创建音乐专辑请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMusicAlbumCmd {
    pub uid: i64,                    // 用户ID
    pub name: Option<String>,        // 名称
    pub name_zh: Option<String>,     // 中文名称
    pub description: Option<String>, // 描述
    pub cover_url: Option<String>,   // 封面url
    pub category_id: Option<i64>,    // 分类ID
    pub tags: Option<String>,        // 标签
    pub source: Option<String>,      // 音乐来源
    pub copyright: Option<String>,   // 版权信息
    pub is_public: Option<bool>,     // 是否公开
}

impl CreateMusicAlbumCmd {
    //

    ////////

    /// # 1. [FROM] - 转换
    /// * `desc`: 创建命令转换为实体
    pub fn to_entity(&self, id_gen: i64) -> MusicAlbumEntity {
        let now = chrono::Utc::now();

        MusicAlbumEntity {
            id: id_gen,
            uuid: Some(Uuid::new_v4().to_string()),
            uid: self.uid,
            name: self.name.clone(),
            name_zh: self.name_zh.clone(),
            description: self.description.clone(),
            cover_url: self.cover_url.clone(),
            music_qty: Some(0), // 新建专辑默认音乐数量为 0
            category_id: self.category_id,
            tags: self.tags.clone(),
            source: self.source.clone(),
            copyright: self.copyright.clone(),
            views: 0,
            likes: 0,
            collects: 0,
            shares: 0,
            review_reason: None,
            status: 1, // 默认状态（1-正常，或根据业务设为 0-草稿）
            is_public: self.is_public,
            is_deleted: Some(false),
            created_at: Some(now),
            updated_at: Some(now),
            deleted_at: None,
            reviewed_at: None,
        }
    }
}

//////// END
