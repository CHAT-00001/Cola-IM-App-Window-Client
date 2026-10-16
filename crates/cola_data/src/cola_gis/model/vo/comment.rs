// vo/add  -- VO - 评论
// 2026/5/22 15:23

////////

use cola_data::app::page::PageInfo;
use cola_data::cola_video::info::comment::CommentInfo;
use serde::Serialize;

////////

/// # [VO] - 评论 视图对象
#[derive(Serialize, Debug)]
pub struct CommentVo {
    // 平铺评论
    #[serde(flatten)]
    pub comment: CommentInfo, // 评论 元信息
    pub is_author: bool,   // 🌟 已修正：平铺在这里，标识当前评论者是否是视频作者
    pub is_liked: bool,    // 是否点赞
    pub is_disliked: bool, // 是否不喜欢
}

/// # [BUILD] - 构造函数
impl CommentVo {
    /// 从已有的 CommentInfo 组装成最终的 VO 对象
    pub fn from_info(
        comment: CommentInfo, // 🌟 去掉了 mut，因为不需要修改内部字段了
        video_author_id: i64,      // 传入视频作者的 UID
        is_liked: bool,
        is_disliked: bool,
    ) -> Self {
        // 🌟 已修正：在这里进行业务逻辑判断，算完直接塞给 VO 实体
        let is_author = comment.user_id == video_author_id;

        Self {
            comment,
            is_author, // 🌟 组装新字段
            is_liked,
            is_disliked,
        }
    }
}

/// # [RESPONSE] - 单条评论响应 (适用于：发布评论成功后返回、查看单条评论详情)
#[derive(Serialize, Debug)]
pub struct CommentSingleResponse {
    pub info: CommentVo,
}

/// # [RESPONSE] - 评论列表响应 (适用于：视频底下的评论区分页列表)
#[derive(Serialize, Debug)]
pub struct CommentListResponse {
    pub comments: Vec<CommentVo>, // 评论列表
    pub page_info: PageInfo,      // 分页信息
}

//////// END
