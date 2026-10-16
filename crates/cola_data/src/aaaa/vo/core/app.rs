// cola_data/src/cola_infra/response/core/app.rs -- DATA - INFRA - response - 核心 - 应用响应
// 2026/6/19 17:35 Created.

////////

use crate::cola_user::info::user::UserInfo;
use crate::cola_user::vo::user::UserVo;
use crate::video::info::video::VideoInfo;
use crate::music::info::music::MusicInfo;
use crate::music::vo::music::MusicVo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 基础设施 - 核心应用视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVo {
    #[serde(flatten)]
    pub info: VideoInfo, // 视频信息
    pub user: UserVo,     // 作者VO
    pub music: MusicVo,   // 音乐VO
    pub is_like: bool,    // 是否点赞
    pub is_collect: bool, // 是否收藏
}

impl AppVo {
    /// # [COMBINE] - 组装核心
    /// 将底层领域数据(Info)转化为前端所需的高级视图对象(Vo)
    pub fn combine(video_info: VideoInfo, author_info: UserInfo, music_info: MusicInfo) -> Self {
        // 1. 克隆一份给音乐 VO 使用（或者如果 music_info 自带 uid 也可以单独查，这里直接共用或给默认值）
        let music_author = author_info.clone();

        Self {
            info: video_info,
            // 作者 VO
            user: UserVo::new(author_info, false, false, false),
            // 🎵 音乐 VO：使用 MusicVo 提供的 combine 构造函数
            music: MusicVo::combine(music_info, music_author),
            // 交互状态兜底
            is_like: false,
            is_collect: false,
        }
    }
}

//////// END
