// case/buy.rs  -- 用例层 - 购买
// 2026/6/10 08:41

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::video::command::buy::BuyCommand;
use repo::video::service::like::LikeService;
use tracing::{info, warn};

////////

/// # [USE CASE] - 购买 用例
pub struct BuyCase;

impl BuyCase {
    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_video_buy(
        uid: i64,
        video_id: i64,
        cmd: BuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // 1. 保存购买记录
        ctx.video
            .buy
            .save_buy_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("添加购买记录失败: {}", e))?;

        // 2. 校验

        // 3. 执行收藏

        Ok(())
    }

    ////////

    /// # 1. [CASE] - 删除
    pub async fn case_del_video_buy(
        uid: i64,
        video_id: i64,
        cmd: BuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // 1. 保存购买记录
        ctx.video
            .buy
            .del_buy_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("删除购买记录数失败: {}", e))?;

        // 2. 校验

        // 3. 执行收藏

        Ok(())
    }
}

//////// END
