// cola_data/src/video/vo/danmaku.rs -- 数据 - VIDEO - vo - 弹幕视图
// 2026/8/8 08:24 Created.

/////////

use crate::app::page::PageInfo;
use crate::app::response::ListResponse;
use crate::video::info::danmaku::DanmakuInfo;
use serde::Serialize;

/////////

/// # [RESPONSE] - 单弹幕响应
#[derive(Debug, Serialize)]
pub struct DanmakuSingleResponse {
    pub info: DanmakuInfo, // 吐给前端完美的、组装好的 VO 列表
}

////////

// /// # [RESPONSE] - 多弹幕响应
// #[derive(Debug, Serialize)]
// pub struct DanmakuListResponse {
//     pub list: Vec<DanmakuInfo>, // 吐给前端完美的、组装好的 VO 列表
//     pub page_info: PageInfo,
// }
//
// // 构造实现
// impl ListResponse<T> {
//     /// ✅ 创建一个空的视频列表响应
//     pub fn empty() -> Self {
//         Self {
//             list: Vec::new(),
//             page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
//         }
//     }
// }
//
// impl Default for ListResponse<T> {
//     fn default() -> Self {
//         Self::empty()
//     }
// }

//////// END
