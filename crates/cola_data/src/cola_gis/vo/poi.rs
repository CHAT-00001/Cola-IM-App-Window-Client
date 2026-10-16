// data/src/cola_gis/vo/poi.rs
// 🗄 数据 - 可乐GIS - vo - POI
// 2026/6/19 17:35 Created.

////////

use crate::cola_user::vo::user::UserVo;
use crate::video::info::video::VideoInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - POI 视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoiVo {
    #[serde(flatten)]
    pub info: VideoInfo,
    pub user: UserVo,
    pub is_like: bool,
    pub is_collect: bool,
}
