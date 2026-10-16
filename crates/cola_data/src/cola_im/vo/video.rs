// data/src/cola_im/vo/new.rs
// 🗄 数据 - 可乐IM - VO - 群
// 2026/6/19 17:35

////////

use crate::cola_user::vo::user::UserVo;
use crate::video::info::video::VideoInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 群 视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupVo {
    #[serde(flatten)]
    pub info: VideoInfo,
    pub user: UserVo,
    pub is_like: bool,
    pub is_collect: bool,
}
