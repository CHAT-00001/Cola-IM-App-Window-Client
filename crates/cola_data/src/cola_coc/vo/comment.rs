// data/src/video/vo/file.rs
// 数据 - ▶ VIDEO - VO - 评论视图对象
// 2026/8/8 14:05 Created.

////////

use crate::app::page::PageInfo;
use crate::video::info::comment::CommentInfo;
use serde::Serialize;

////////

/// # [VO] - 单评论响应（已废弃，请使用泛型）
#[deprecated(since = "2026-08-08", note = "Use ListResponse<CommentVo> instead")]
#[derive(Debug, Serialize)]
pub struct CommentSingleResponse {
    pub info: CommentInfo, // 吐给前端完美的、组装好的单体 VO
}

////////

/// # [VO] - 多评论响应
pub struct CommentListResponse {
    pub list: Vec<CommentInfo>,
    pub page: PageInfo,
}

//////// END
