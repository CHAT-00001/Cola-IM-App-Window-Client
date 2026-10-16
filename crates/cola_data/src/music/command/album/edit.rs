// cola_data/src/music/cmd/album/edit.rs
// 数据中心 - MUSIC - cmd - 编辑专辑
// 2026/8/3 21:46 Created.

////////

use crate::music::entity::album::MusicAlbumEntity;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 编辑
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMusicAlbumCmd {
    pub id: i64,                     // 专辑ID
    pub name: Option<String>,        // 名称
    pub name_zh: Option<String>,     // 中文名称
    pub description: Option<String>, // 描述
    pub cover_url: Option<String>,   // 封面url
    pub category_id: Option<i64>,    // 分类ID
    pub tags: Option<String>,        // 标签
    pub copyright: Option<String>,   // 版权信息
    pub is_public: Option<bool>,     // 是否公开
    pub status: Option<i16>,         // 状态
}

// 构造实现
impl UpdateMusicAlbumCmd {
    //

    ////////

    /// # 1. [APPLY] - 应用
    /// * `desc`: 将编辑命令应用到现有的 MusicAlbumEntity 实体中（修改标题、封面等）
    pub fn apply_to(&self, entity: &mut MusicAlbumEntity) {
        if let Some(ref name) = self.name {
            entity.name = Some(name.clone());
        }
        if let Some(ref name_zh) = self.name_zh {
            entity.name_zh = Some(name_zh.clone());
        }
        if let Some(ref description) = self.description {
            entity.description = Some(description.clone());
        }
        if let Some(ref cover_url) = self.cover_url {
            entity.cover_url = Some(cover_url.clone());
        }
        if let Some(category_id) = self.category_id {
            entity.category_id = Some(category_id);
        }
        if let Some(ref tags) = self.tags {
            entity.tags = Some(tags.clone());
        }
        if let Some(ref copyright) = self.copyright {
            entity.copyright = Some(copyright.clone());
        }
        if let Some(is_public) = self.is_public {
            entity.is_public = Some(is_public);
        }
        if let Some(status) = self.status {
            entity.status = status;
        }

        // 更新时间戳
        entity.updated_at = Some(chrono::Utc::now());
    }
}

//////// END
