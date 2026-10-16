// case/hotlist.rs  -- 用例层 - 上热门
// 2026/6/10 08:37

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::video::command::hotlist::HotlistCommand;
use repo::video::service::like::LikeService;
use tracing::{info, warn};

////////

/// # [CASE] - 上热门 用例
pub struct HotlistCase;

impl HotlistCase {
    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_hotlist(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: HotlistCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // 1. 保存上热门(抖+)记录
        ctx.video
            .hotlist
            .save_hotlist_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("检查视频健康状态失败: {}", e))?;

        Ok(())
    }

    ////////
}

//////// END
