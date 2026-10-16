// case/report.rs  -- 用例层 举报
// 2026/6/10 19:20

////////

use crate::assembler::video::build_video_list_response;
use crate::model::vo::video::{VideoListResponse, VideoSingleResponse};
use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::risk::rick_check;
use cola_data::video::command::report::ReportCommand;
use futures_util::TryFutureExt;
use repo::video::service::home::VideoHomeService;
use tracing::info;
////////

/// # [REPORT CASE] - 举报 用例
pub struct ReportCase;

// 构造函数
impl ReportCase {
    // * --------

    ////////

    /// # 1. [CASE]] - 举报
    pub async fn case_add_report(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: ReportCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        ctx.video
            .report
            .save_report_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 保存举报记录失败: {}", e))?;

        info!("BIZ - 保存举报记录成功: uid={}", uid);
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 获取被举报的视频列表
    pub async fn case_get_report_video(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // Call Service: 解构元组，同时拿到总数 total 和 视频 IDs 数组
        let (total, video_ids) = ctx
            .video
            .report
            .get_report_record_ids(url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取举报记录ID失败: {}", e))?;

        // 如果没有被举报的视频，直接返回默认的空响应
        if video_ids.is_empty() {
            return Ok(VideoListResponse::default());
        }

        // Repo: 🌟 顺次升级！用 ids 批量拿到纯净的领域对象 VideoInfo 列表
        let video_infos = ctx
            .video
            .view
            .get_video_list_by_ids(video_ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 批量获取视频信息失败: {}", e))?;

        // Transform: 🌟 完美对齐 5 参数装配器！
        // 传入拿到的总数 total，让前端的分页器完美生效
        let response = build_video_list_response(
            video_infos, // 1. 视频 Info 列表
            None,        // 2. uid (这里是后台管理或不需要操作者上下文，传 None)
            url.offset,  // 3. 偏移量
            url.limit,   // 4. 每页限制
            total,       // 5. 🌟 真正的总条数
        )
        .await // 别忘了我们的装配器是 async 的
        .map_err(|e| anyhow::anyhow!("BIZ: 组装视频列表响应体失败: {}", e))?;

        tracing::info!("DOMAIN: 获取被举报的视频列表成功啦~! 总数: {}", total);

        Ok(response)
    }

    ////////
}

//////// END
