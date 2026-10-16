// case/code  -- 用例层 - 发布
// 2026-06-11 修正风控返回值与单视频响应组装

use crate::assembler::video::build_video_single_response;
use crate::model::vo::video::VideoSingleResponse;
use anyhow::{Context, Result};
use cola_data::risk::rick_check;
use cola_data::video::command::video::VideoCommand;
use repo::video::service::add::AddService;
use tracing::info;

////////

pub struct AddCase;

impl AddCase {
    ////////

    /// # 1. [CASE] - 发布视频
    pub async fn case_add_publish(
        uid: i64,
        cmd: VideoCommand,
    ) -> Result<VideoSingleResponse, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{} {:?}", cmd.title, cmd.description);

        // ✅ 核心修复：rick_check 异步执行后出来就是 i16，直接 await 拿值，删掉多余的 map_err!?
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新 (💡 提示：建议让这个 Service 函数返回刚插入成功的 VideoInfo)
        let video_info = AddService::save_video_and_update_count(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 视频发布持久化失败: {}", e))?;

        info!("BIZ - 视频发布成功: uid={}, visibility={}", uid, visibility);

        // 3. 🌟 架构对齐：用我们刚才写好的高质量总装器，动态拼装博主信息后返回给前端
        let response = build_video_single_response(video_info, Some(uid)).await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 编辑视频
    pub async fn case_edit_publish(
        uid: i64,
        cmd: VideoCommand,
    ) -> Result<VideoSingleResponse, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{} {:?}", cmd.title, cmd.description);

        // ✅ 核心修复：同上，直接接住 i16
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新
        let video_info = AddService::edit_video(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 视频发布持久化失败: {}", e))?;

        info!("BIZ - 视频发布成功: uid={}, visibility={}", uid, visibility);

        // 3. 🌟 架构对齐：用我们刚才写好的高质量总装器，动态拼装博主信息后返回给前端
        let response = build_video_single_response(video_info, Some(uid)).await?;

        Ok(response)
    }

    ////////
}

//////// END
