// cola_data/src/video/response/video_resp.rs  -- DATA - VIDEO - RESPONSE - 视频响应体
// 2026/5/21 03:47

////////

use crate::app::page::PageInfo;
use crate::cola_user::info::user::UserInfo;
use crate::video::info::video::VideoInfo;
use crate::music::info::music::MusicInfo;
use serde::Serialize;

////////

/// # [MODEL] - 视频视图模型
#[derive(Debug, Serialize, Clone)]
pub struct VideoVo {
    // 🌟 直接扁平化展开（Flatten），前端甚至感觉不到结构体拆分了！
    // 访问时依然是 video.id, video.title，不需要 video.info.title
    #[serde(flatten)]
    pub video: VideoInfo,

    // 动态组装的关联数据
    pub user_info: UserInfo,
    pub music_info: MusicInfo,

    // 针对当前登录用户的动态交互显示
    pub is_visited: bool,
    pub is_like: bool,
    pub is_collect: bool,
    pub is_push: bool,
}

/// # [BUILD] - 构建视频视图
impl VideoVo {
    /// 在 BIZ 层进行大聚合的构造函数
    pub fn combine(
        video: VideoInfo,
        user: UserInfo,
        music: MusicInfo,
        // 也可以选择把交互状态作为参数传进来，或者提供 set 方法
    ) -> Self {
        Self {
            video,
            user_info: user,
            music_info: music,
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

impl Default for VideoVo {
    fn default() -> Self {
        Self {
            video: VideoInfo::empty(),
            user_info: UserInfo::default(),
            music_info: MusicInfo::default(),
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

/// # [RESPONSE] - 单视频响应
#[derive(Debug, Serialize)]
pub struct VideoSingleResponse {
    pub info: VideoVo, // 吐给前端完美的、组装好的 VO 列表
}

/// # [RESPONSE] - 多视频响应
#[derive(Debug, Serialize)]
pub struct VideoListResponse {
    pub list: Vec<VideoVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 🌟🌟🌟 新增：最小侵入性扩展，为多视频响应提供空响应构造 🌟🌟🌟
impl VideoListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for VideoListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
