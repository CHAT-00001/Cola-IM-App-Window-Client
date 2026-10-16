// case/collect.rs  -- 用例层 - 收藏
// 2026/6/10 07:00

////////

use anyhow::Result;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::video::VideoCommand;
use repo::video::service::collect::CollectService;
use repo::video::service::permission_change::PermissionsChangeService;
use tracing::{info, warn};

////////

/// # [USE CASE] - 收藏 用例
pub struct CollectCase;

impl CollectCase {
    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_collect(uid: i64, video_id: i64, cmd: CollectCommand) -> Result<bool> {
        // 修改视频评论权限
        CollectService::save_collect_and_update_count(uid, video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 添加收藏失败: {}", e))?;

        info!("BIZ - 添加收藏成功: uid={}, video_id={}, ", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 2. [CASE] - 编辑
    pub async fn case_set_collect(uid: i64, video_id: i64, cmd: CollectCommand) -> Result<bool> {
        // 修改视频评论权限
        CollectService::save_collect_and_update_count(uid, video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 编辑失败: {}", e))?;

        info!("BIZ - 编辑成功: uid={}, video_id={}, ", uid, video_id);
        Ok(true)
    }

    ////////

    ////////

    /// # 4. [CASE] - 删除一个
    pub async fn case_del_collect(
        uid: i64,
        video_id: i64,
        // collect_id: i64,
    ) -> Result<bool> {
        // 删除收藏
        CollectService::del_collect_and_update_count(uid, video_id)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 删除失败: {}", e))?;

        info!("BIZ - 删除成功: uid={}, video_id={}, ", uid, video_id,);
        Ok(true)
    }

    ////////
}

//////// END
