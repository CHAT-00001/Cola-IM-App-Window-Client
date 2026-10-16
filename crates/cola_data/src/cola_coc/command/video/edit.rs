// cola_data/src/video/cmd/video/edit.rs  --
// 数据 - VIDEO - cmd - video - edit 编辑内容
// 2026/8/1 18:18

////////

use crate::video::entity::video::video::VideoEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 视频编辑命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUpdateCommand {
    pub id: i64,                     // 视频 ID
    pub title: String,               // 新标题
    pub description: Option<String>, // 新描述
    pub cover_url: String,           // 新封面
}

impl VideoUpdateCommand {
    /// 将编辑 Command 应用并更新到现有的 VideoEntity 中
    pub fn update_entity(self, entity: &mut VideoEntity) {
        // ⏰️ 获取当前时间戳用于更新 updated_at
        let now = Utc::now();

        // 更新可编辑字段
        entity.title = self.title;
        entity.description = self.description;
        entity.thumbnail = Some(self.cover_url);

        // 🔄 同步更新修改时间
        entity.updated_at = Some(now);
    }
}

//////// END
