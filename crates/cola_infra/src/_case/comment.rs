// case/comment.rs  -- 用例层 - 评论
// 2026/5/23 15:20

////////

use crate::assembler::comment::{build_comment_list_response, build_comment_single_response};
use crate::model::vo::comment::{CommentListResponse, CommentSingleResponse, CommentVo};
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use cola_data::video::command::comment::CommentCommand;
use repo::video::service::comment::CommentService;
use std::collections::HashMap;

////////

/// # [COMMENT CASE] - 评论 用例
pub struct CommentCase;

////////
impl CommentCase {
    ////////

    /// # 1. [CASE] - 发布
    /// * `描述` 用户UGC评论应用编排
    pub async fn case_add_comment(
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
        ctx: &AppContext,
    ) -> Result<CommentSingleResponse> {
        // 1. 检查评论内容风控等级
        let key = cmd.content.to_string();
        let visibility = 5;

        // 2. 调用SERVICE
        // * 拿到评论信息
        let infos =
            CommentService::save_comment_and_update_count(uid, video_id, cmd, visibility).await?;

        // 3. 组装成vo
        let response = build_comment_single_response(infos, Some(uid)).await?;

        // 3. 返回成功给API层响应
        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 浏览视频评论列表
    pub async fn case_get_video_comments_list(
        uid: i64,
        video_id: i64,
        query: ApiUrlParamsQuery,
        _ctx: &AppContext,
    ) -> Result<CommentListResponse> {
        // 1. ✅ 核心修复：干掉元组解构，直接用单变量接住 Vec<CommentInfo>
        let infos =
            CommentService::find_comments_by_video_id(video_id, query.offset, query.limit).await?;

        // 2. 拿到当前页的局部数量，用来给下层组装的分页算总量兜底
        let current_page_total = infos.len() as i64;

        // 3. 直接调用组装器
        let response = build_comment_list_response(
            infos,
            Some(uid),
            query.page.unwrap_or(1),
            query.qty.unwrap_or(10),
            current_page_total,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 3. [CASE] - 浏览我的发布评论列表
    /// * 描述: 获取我发布的评论列表
    /// * `uid` 我的用户ID
    /// * `query` 查询参数
    pub async fn case_get_user_comments_list(
        uid: i64,                 // 操作者 ID
        query: ApiUrlParamsQuery, // url查询参数
        ctx: &AppContext,
    ) -> Result<CommentListResponse> {
        // Call Service
        let infos =
            CommentService::find_comments_by_user_id(uid, query.limit, query.offset).await?;

        // 2. 拿到当前页的局部数量，用来给下层组装的分页算总量兜底
        let current_page_total = infos.len() as i64;

        let response = build_comment_list_response(
            infos,
            Some(uid),
            query.page.unwrap_or(1),
            query.qty.unwrap_or(10),
            current_page_total,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 4. [CASE] - 删除一条评论
    pub async fn case_del_one_item(
        uid: i64,
        comment_id: i64, // 评论 ID
        snowflake: i64,  // 雪花 ID
        ctx: &AppContext,
    ) -> Result<(String)> {
        ctx.video.comment.del_comment_record(comment_id).await?;
        Ok("单条删除评论成功~".to_string())
    }

    /////////

    /// # 5. [CASE] - 批量删除评论
    pub async fn case_del_all_item(comment_ids: Vec<i64>, ctx: &AppContext) -> Result<(String)> {
        ctx.video.comment.del_comments_record(comment_ids).await?;
        Ok("批量删除评论成功~".to_string())
    }

    ////////

    /// # 6. [CASE] - 点赞
    /// * `描述` : 用户点赞一条评论
    pub async fn case_add_comment_like(uid: i64, comment_id: i64, is_liked: bool) -> Result<()> {
        CommentService::update_comment_like_by_id(uid, comment_id, is_liked).await?;
        Ok(())
    }

    /////////

    /// # 7. [CASE] - 不喜欢
    /// * `描述` : 用户不喜欢一条评论
    pub async fn case_add_comment_unlike(
        uid: i64,
        comment_id: i64,
        is_unliked: bool,
        ctx: &AppContext,
    ) -> Result<()> {
        CommentService::update_comment_unlike_by_id(uid, comment_id, is_unliked).await?;
        Ok(())
    }

    ////////

    /// # 8. [CASE] - 举报
    /// * `描述` : 用户举报一条评论
    /// * `uid` 我的用户ID
    /// * `comment_id` 查询参数
    /// * `is_liked` 是否点赞
    pub async fn case_add_comment_report(uid: i64, comment_id: i64, is_liked: bool) -> Result<()> {
        CommentService::update_comment_like_by_id(uid, comment_id, is_liked).await?;
        Ok(())
    }

    ////////
}

//////// END
