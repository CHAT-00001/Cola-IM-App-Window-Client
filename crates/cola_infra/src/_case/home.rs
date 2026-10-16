// /case/home.rs  -- 可乐短视频 - 用例层 - home
// 2026-06-11 08:10

////////

use crate::assembler::video::build_video_list_response;
use crate::model::vo::video::VideoListResponse;
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::user::info::config::UserConfigInfo;
use repo::video::service::home::VideoHomeService;

////////

/// # [HOME CASE] - 主页 用例
pub struct HomeCase;

impl HomeCase {
    ////////

    /// # 1. [CASE] - 配置 (保持原样，不处理视频)
    pub async fn case_get_con(
        uid: i64, // 操作者 ID
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<UserConfigInfo> {
        // Call Port
        let entities = ctx.user.con.get_config(uid).await?;
        Ok(entities)
    }

    ////////

    /// # 2. [CASE] - 新的
    pub async fn case_get_new_list(
        uid: i64, // 操作者 ID
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let video_infos = VideoHomeService::find_new_video_list(url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            Some(uid),
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 3. [CASE] - 热门
    pub async fn case_get_hot_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let video_infos = VideoHomeService::find_hot_video_list(url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 4. [CASE] - 推荐
    pub async fn case_get_recommend_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let video_infos =
            VideoHomeService::find_recommend_video_list(url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 5. [CASE] - 同城
    pub async fn case_get_city_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 🌟 已修正：下层直接返回 info，去掉多余转换逻辑
        let video_infos =
            VideoHomeService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    /// # 6. [CASE] - 分类
    pub async fn case_get_category_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 🌟 已修正：直接一步到位拿到 video_infos
        let video_infos =
            VideoHomeService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 7. [CASE] - 附近
    pub async fn logic_get_nearby_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 🌟 已修正：直接一步到位拿到 video_infos
        let video_infos =
            VideoHomeService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    //////

    /// # 8. [CASE] - 精选
    pub async fn case_get_featured_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let video_infos = VideoHomeService::find_featured_video_list(url.limit, url.offset).await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }

    /// # 9. [CASE] - 搜索
    pub async fn case_get_keyword_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 🌟 已修正：去掉底层的 Row 解构，直接接收干净的 video_infos
        let video_infos = VideoHomeService::search_video_keyword_list(
            url.keyword,
            lat,
            lng,
            url.limit,
            url.offset,
        )
        .await?;

        let response = build_video_list_response(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
        .await?;

        Ok(response)
    }
}
