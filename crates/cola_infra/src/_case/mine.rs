// case/mine.rs  -- 用例层 - 我的
// 2026/06/11 架构对齐重构

////////

use crate::assembler::video::build_video_list_response;
use crate::model::vo::video::VideoListResponse;
use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use repo::video::service::like::LikeService;
use repo::video::service::view::ViewService;

////////

pub struct MineCase;

// 构造函数
impl MineCase {
    ////////

    /// # 1. [CASE] - 关注的人发布的视频
    pub async fn case_feed_following(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // 1. following
        let ids = ctx.user.following.get_following_ids(uid).await?;

        // CALL SERVICE
        let infos =
            ViewService::batch_uids_get_videos_infos(ids, Some(url.keyword), url.offset, url.limit)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("BIZ: 获取用户 {} 视频列表失败: {}", url.user_id, e)
                })?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 2. [CASE] - 朋友发布的
    pub async fn case_feed_friend(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // 1. friend
        let ids = ctx.user.following.get_following_ids(uid).await?;

        // CALL SERVICE
        let infos =
            ViewService::batch_uids_get_videos_infos(ids, Some(url.keyword), url.offset, url.limit)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("BIZ: 获取用户 {} 视频列表失败: {}", url.user_id, e)
                })?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 3. [CASE] - 为我推荐的
    pub async fn case_feed_recommend(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        let ids = ctx.user.following.get_following_ids(uid).await?;

        // CALL SERVICE
        let infos = ViewService::batch_get_videos_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 视频列表失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 4. [CASE] - TA发布的
    pub async fn case_feed_publish(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // CALL SERVICE
        let infos = ViewService::get_videos_infos_by_uid(
            url.user_id,
            Some(url.keyword),
            url.offset,
            url.limit,
        )
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 发布的视频列表失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 5. [CASE] - TA点赞的
    pub async fn case_feed_liked(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // CALL SERVICE
        let ids = LikeService::get_user_like_ids(url.user_id, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        // CALL SERVICE
        let infos = ViewService::batch_get_videos_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }

    ////////

    /// # 6. [CASE] - TA收藏的
    pub async fn case_feed_collected(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // CALL SERVICE
        let ids = LikeService::get_user_like_ids(url.user_id, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        // CALL SERVICE
        let infos = ViewService::batch_get_videos_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 收藏的列表失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }

    ////////

    /// # 7. [CASE] - TA推荐的
    pub async fn case_user_recommend(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        let ids = LikeService::get_user_like_ids(url.user_id, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        // CALL SERVICE
        let infos = ViewService::batch_get_videos_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 推荐的列表失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }

    ////////

    /// # 8. [CASE] - TA附近的视频
    pub async fn case_feed_nearby(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<VideoListResponse> {
        // 模拟目标用户位置
        let lat = url.lat.unwrap_or(0.0);
        let lng = url.lng.unwrap_or(0.0);
        let range = 50000.0;

        let infos = ctx
            .video
            .feed
            .get_nearby_list(lat, lng, range, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 附近视频失败: {}", url.user_id, e))?;

        let resp =
            build_video_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }
}

//////// END
