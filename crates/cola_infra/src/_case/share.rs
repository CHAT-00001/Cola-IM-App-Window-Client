// video/src/case/share.rs  -- 用例层 - 分享
// 2026/6/10 08:14

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::video::command::share::ShareCommand;
use repo::video::service::like::LikeService;
use tracing::{info, warn};

////////

////////

/// # [APP USE CASE] - 分享用例
pub struct ShareCase;

impl ShareCase {
    ////////

    /// # 1. [CASE] - 添加分享
    pub async fn case_add_video_share(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: ShareCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // 1. 获取当前收藏数 (自动从存储层获取)
        ctx.video
            .share
            .save_share_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("保存分享视频记录失败:  {}", e))?;

        Ok(())
    }

    /// # 2. [CASE] - 移除分享
    pub async fn case_remove_collect_share(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<()> {
        ctx.video
            .share
            .delete_share_record(uid, url.video_id)
            .await
            .map_err(|e| anyhow!("删除分享记录失败: {}", e))?;
        Ok(())
    }
}

//////// END
