// cola_data/src/video/cmd/video/status.rs  --
// 数据 - VIDEO - cmd - video - edit 修改状态
// 2026/8/1 18:24

////////

use crate::video::entity::video::video::VideoEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 视频 修改状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUpdateStatusCommand {
    pub id: i64,     // 视频 ID
    pub status: i16, // 状态码
}

impl VideoUpdateStatusCommand {
    //

    ////////

    /// # [BUILD] - 修改状态
    /// * `desc`: `将编辑 Command 应用并更新到现有的 VideoEntity 中`
    pub fn update_entity(self, entity: &mut VideoEntity) {
        // ⏰️ 获取当前时间戳用于更新 updated_at
        let now = Utc::now();

        // 修改状态码
        entity.status = self.status;

        // 🔄 同步更新修改时间
        entity.updated_at = Option::from(now);
    }
}

//////// END
