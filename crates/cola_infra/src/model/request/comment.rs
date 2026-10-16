// cola_infra/src/model/request/comment.rs  -- 请求体 - 评论
// 2026/6/10 02:26

////////

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

////////

/// # [REQUEST] - 评论请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentRequest {
    pub comment_id: i64, // 评论 ID
    pub user_id: i64,    // 用户 ID
    pub video_id: i64,   // 视频 ID
    pub keyword: String, // 关键词
}

//  构造函数
impl CommentRequest {
    /// 空上下文（内部调用）
    pub fn empty() -> Self {
        Self::default()
    }

    /// 已登录上下文
    /// 必须登录，否则返回错误（统一 AppData 体系）
    pub fn ok() -> Self {
        Self::default()
    }
}
