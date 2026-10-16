// cola_data/src/cola_user/response/category.rs  -- USER - Response - 分类
// 2026/5/22 19:51 by wx: cestbon10080

////////

use crate::app::page::PageInfo;
use serde::{Deserialize, Serialize};
////////

/// # [VO] - 短视频 - 分类视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCategoryVo {
    pub id: i64,      // 视频分类 ID
    pub name: String, // 视频分类名称
}

////////

/// # [VO] - 短视频 - 单个分类视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCategorySingleResponse {
    pub category: VideoCategoryVo, // 分类视图对象
}

////////

/// # [VO] - 短视频 - 列表分类视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCategoryListResponse {
    pub categories: Vec<VideoCategoryVo>, // 分类视图对象
    pub page_info: PageInfo,              // 分页信息
}
