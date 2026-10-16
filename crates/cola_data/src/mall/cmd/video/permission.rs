// cola_data/src/video/cmd/video/permission.rs  --
// 数据 - VIDEO - cmd - video - permission 修改权限
// 2026/8/1 18:23

////////

use crate::video::entity::video::video::VideoEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 视频权限修改命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUpdatePermissionCommand {
    pub id: i64,              // 视频 ID
    pub visibility_perm: i16, // 可见权限
    pub comment_perm: i16,    // 评论权限
    pub danmaku_perm: i16,    // 弹幕权限
    pub collect_perm: i16,    // 收藏权限
    pub download_perm: i16,   // 下载权限
}

// 构造实现
impl VideoUpdatePermissionCommand {
    //

    ////////

    /// # [BUILD] - 修改权限
    /// 将编辑 Command 应用并更新到现有的 VideoEntity 中
    pub fn update_entity(self, entity: &mut VideoEntity) {
        // ⏰️ 获取当前时间戳用于更新 updated_at
        let now = Utc::now();

        // 更新权限相关字段
        entity.visibility_perm = self.visibility_perm;
        entity.comment_perm = self.comment_perm;
        entity.danmaku_perm = self.danmaku_perm;
        entity.collect_perm = self.collect_perm;
        entity.download_perm = self.download_perm;

        // 🔄 同步更新修改时间
        entity.updated_at = Option::from(now);
    }
}

//////// END
