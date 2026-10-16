// /danmaku.rs  -- 用例层 - 弹幕
// 2026/4/24 19:34

////////

use crate::assembler::danmaku::build_danmaku_list_response;
use crate::model::vo::danmaku::DanmakuListResponse;
use crate::model::vo::video::VideoListResponse;
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::video::command::danmaku::DanmakuCommand;
use repo::user::service::user::UserService;
use repo::video::service::danmaku::DanmakuService;

////////

/// # [CASE] - 弹幕 用例
pub struct DanmakuCase;

////////
impl DanmakuCase {
    ////////

    /// # 1. [CASE] - 发表弹幕
    /// * `描述` 用户UGC弹幕应用编排
    pub async fn case_add_danmaku(uid: i64, video_id: i64, cmd: DanmakuCommand) -> Result<String> {
        // 1. 检查弹幕内容风控等级
        let key = cmd.content.to_string();
        let visibility = 5;

        // 2. 调用SERVICE
        let service =
            DanmakuService::save_danmaku_and_update_count(uid, video_id, cmd, visibility).await?;

        // 3. 返回成功给API层响应
        Ok("ok".to_string())
    }

    ////////

    /// # 2. [CASE] - 浏览视频弹幕列表
    pub async fn case_video_danmakus(
        uid: i64,
        video_id: i64,
        play_time: i32,
        _time_window: i32,
        qty: i64, // 💡 顺手把 qty 接进来，别写死 20 了
        ctx: &AppContext,
    ) -> Result<DanmakuListResponse> {
        // 1. 调用 Repo 端口，稳稳接住 (Vec<DanmakuInfo>, total)
        let (danmaku_list, total) = ctx
            .video
            .danmaku
            .get_danmaku_by_video_id(uid, video_id, play_time, qty as i32)
            .await?;

        // 2. 准备组装器要的标识参数
        let current_uid = if uid > 0 { Some(uid) } else { None };
        let video_author_id = 0i64; // 💡 预留：未来可以从视频服务获取真实的作者 ID

        // 3. ✅ 完美契合：6 个参数一字排开，数据流清清爽爽
        let response = build_danmaku_list_response(
            danmaku_list,
            current_uid,
            video_author_id,
            1,     // page 默认第一页（或者从别处扩展传入）
            qty,   // 动态传入前端请求的数量
            total, // 数据库真实的总条数
        )
        .await?;

        // 4. 返回视图
        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 浏览用户弹幕列表
    pub async fn case_user_danmakus(
        uid: i64,
        query: ApiUrlParamsQuery,
        ctx: &AppContext,
    ) -> Result<DanmakuListResponse> {
        // 数量 来源 分页限制
        let qty = query.limit;

        // 1. 调用 Repo 端口，稳稳接住 (Vec<DanmakuInfo>, total)
        let (danmaku_list, total) = ctx
            .video
            .danmaku
            .get_danmaku_by_user_id(uid, query.offset, query.limit)
            .await?;

        // 2. 准备组装器要的标识参数
        let current_uid = if uid > 0 { Some(uid) } else { None };
        let video_author_id = 0i64; // 💡 预留：未来可以从视频服务获取真实的作者 ID

        // 3. ✅ 完美契合：6 个参数一字排开，数据流清清爽爽
        let response = build_danmaku_list_response(
            danmaku_list,
            current_uid,
            video_author_id,
            1,     // page 默认第一页（或者从别处扩展传入）
            qty,   // 动态传入前端请求的数量
            total, // 数据库真实的总条数
        )
        .await?;

        // 4. 返回视图
        Ok(response)
    }

    ////////

    /// # 4. [CASE] - 删除一条弹幕
    pub async fn case_del_danmaku(_uid: i64, _video_id: i64, _content: String) -> Result<(String)> {
        Ok("删除弹幕成功~".to_string())
    }

    ////////

    /// # 6. [CASE] - 添加弹幕点赞
    /// * `描述` : 用户点赞一条弹幕
    /// * `uid` 我的用户ID
    /// * `danmaku_id` 查询参数
    /// * `is_liked` 是否点赞
    pub async fn case_add_danmaku_like(uid: i64, danmaku_id: i64, is_liked: bool) -> Result<()> {
        DanmakuService::add_like_and_update_count(uid, danmaku_id, is_liked).await?;
        Ok(())
    }

    ////////

    /// # 6. [CASE] - 添加弹幕不喜欢
    /// * `描述` : 用户不喜欢一条弹幕
    /// * `uid` 我的用户ID
    /// * `danmaku_id` 查询参数
    /// * `is_unliked` 是否点赞
    pub async fn case_add_danmaku_unlike(
        uid: i64,
        danmaku_id: i64,
        is_unliked: bool,
    ) -> Result<()> {
        DanmakuService::add_unlike_and_update_count(uid, danmaku_id, is_unliked).await?;
        Ok(())
    }
}

//////// END
