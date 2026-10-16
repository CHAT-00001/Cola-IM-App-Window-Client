// cola_data/src/aaaa/vo/comment.rs -- DATA - BASIC - VO - 评论视图
// 2026/8/8 14:05 Created.

////////

use crate::app::page::PageInfo;
use crate::cola_user::vo::user::UserVo;
use crate::video::info::comment::CommentInfo;
use serde::Serialize;

////////

/// # [VO] - 视频评论视图对象
#[derive(Debug, Serialize, Clone)]
pub struct CommentVo {
    #[serde(flatten)] // 平铺评论
    pub info: CommentInfo, // 评论 Info
    pub user: UserVo,     // 用户视图
    pub is_author: bool,  // 是否作者
    pub is_owner: bool,   // 是否持有
    pub is_dislike: bool, // 是否不喜欢
    pub is_like: bool,    // 是否点赞
}

// 构造视图
impl CommentVo {
    //

    ////////

    /// #1. [NEW] - 完整构造函数
    pub fn new(
        info: CommentInfo,
        user: UserVo,
        is_author: bool,
        is_owner: bool,
        is_like: bool,
        is_dislike: bool,
    ) -> Self {
        Self {
            info,
            user,
            is_author,
            is_owner,
            is_like,
            is_dislike,
        }
    }

    /// #2. [BUILD] - 基础默认状态构造（无点赞/非作者归属）
    pub fn from_basic(info: CommentInfo, user: UserVo) -> Self {
        Self {
            info,
            user,
            is_author: false,
            is_owner: false,
            is_dislike: false,
            is_like: false,
        }
    }
}

////////

/// # [RESPONSE] - 单评论响应
#[deprecated(since = "2026-08-08", note = "Use ListResponse<CommentVo> instead")]
#[derive(Debug, Serialize)]
pub struct CommentSingleResponse {
    pub info: CommentVo, // 吐给前端完美的、组装好的单体Vo
}

/// # [RESPONSE] - 多评论响应
#[derive(Debug, Serialize, Clone)]
pub struct CommentListResponse {
    pub list: Vec<CommentVo>,
    pub page: PageInfo,
}

//////// END
