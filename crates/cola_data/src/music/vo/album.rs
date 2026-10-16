// cola_data/src/music/vo/album.rs
// 数据中心 - MUSIC - vo - 专辑视图
// 2026/8/31 03:01 Created.

////////

use crate::app::page::PageInfo;
use crate::cola_user::info::user::UserInfo;
use crate::music::info::album::MusicAlbumInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 音乐专辑视图模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicAlbumVo {
    #[serde(flatten)]
    pub album: MusicAlbumInfo,
    pub user_info: UserInfo,
    pub is_visited: bool,
    pub is_like: bool,
    pub is_collect: bool,
    pub is_push: bool,
}

/// # [BUILD] - 构建视图
impl MusicAlbumVo {
    /// 在 BIZ 层进行大聚合的构造函数
    pub fn combine(
        album: MusicAlbumInfo,
        user: UserInfo,
        // 也可以选择把交互状态作为参数传进来，或者提供 set 方法
    ) -> Self {
        Self {
            album,
            user_info: user,
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

impl Default for MusicAlbumVo {
    //

    ////////

    /// # [BUILD] - 默认
    fn default() -> Self {
        Self {
            album: MusicAlbumInfo::empty(),
            user_info: UserInfo::default(),
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

/// # [RESPONSE] - 单专辑响应
#[derive(Debug, Serialize)]
pub struct MusicAlbumSingleResponse {
    pub info: MusicAlbumVo, // 吐给前端完美的、组装好的 VO 列表
}

/// # [RESPONSE] - 多专辑响应
#[derive(Debug, Serialize)]
pub struct MusicAlbumListResponse {
    pub list: Vec<MusicAlbumVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 🌟🌟🌟 新增：最小侵入性扩展，为多音乐响应提供空响应构造 🌟🌟🌟
impl MusicAlbumListResponse {
    /// ✅ 创建一个空的音乐列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for MusicAlbumListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
