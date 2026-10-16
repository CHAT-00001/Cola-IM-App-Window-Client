// cola_infra/src/model/request/danmaku.rs  -- 弹幕请求体参数
// 2026/6/10 03:11

////////

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

////////

/// # [REQUEST] - 弹幕请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DanmakuRequest {
    pub danmaku_id: Option<i64>,  // 评论 ID
    pub user_id: Option<i64>,     // 用户 ID
    pub video_id: Option<i64>,    // 视频 ID
    pub play_time: Option<i32>,   // 播放时间
    pub time_window: Option<i32>, // 时间窗口
}

//  构造函数
impl DanmakuRequest {
    /// 空上下文
    pub fn empty() -> Self {
        Self::default()
    }

    /// 已登录上下文
    pub fn ok() -> Self {
        Self::default()
    }
}
