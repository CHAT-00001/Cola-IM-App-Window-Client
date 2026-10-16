// cola_data/src/music/vo/music_vo.rs
// 数据中心 - MUSIC - vo - 音乐视图
// 2026/7/7 13:01

////////

use crate::app::page::PageInfo;
use crate::cola_user::info::user::UserInfo;
use crate::music::info::music::MusicInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 音乐视图模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVo {
    #[serde(flatten)]
    pub music: MusicInfo,
    pub user_info: UserInfo,
    pub is_visited: bool,
    pub is_like: bool,
    pub is_collect: bool,
    pub is_push: bool,
}

/// # [BUILD] - 构建视图
impl MusicVo {
    /// 在 BIZ 层进行大聚合的构造函数
    pub fn combine(
        music: MusicInfo,
        user: UserInfo,
        // 也可以选择把交互状态作为参数传进来，或者提供 set 方法
    ) -> Self {
        Self {
            music,
            user_info: user,
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

impl Default for MusicVo {
    //

    ////////

    /// # [BUILD] - 默认
    fn default() -> Self {
        Self {
            music: MusicInfo::empty(),
            user_info: UserInfo::default(),
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

/// # [RESPONSE] - 单音乐响应
#[derive(Debug, Serialize)]
pub struct MusicSingleResponse {
    pub info: MusicVo, // 吐给前端完美的、组装好的 VO 列表
}

/// # [RESPONSE] - 多音乐响应
#[derive(Debug, Serialize)]
pub struct MusicListResponse {
    pub list: Vec<MusicVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 🌟🌟🌟 新增：最小侵入性扩展，为多音乐响应提供空响应构造 🌟🌟🌟
impl MusicListResponse {
    /// ✅ 创建一个空的音乐列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for MusicListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
