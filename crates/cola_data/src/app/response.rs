// cola_data/src/app/response.rs -- DATA - APP - 响应体
// 2026/7/27 13:30 Created.

////////

use crate::app::page::PageInfo;
use serde::Serialize;

////////

/// # [RESPONSE] - 单个实体响应
#[derive(Debug, Serialize)]
pub struct SingleResponse<T: Serialize> {
    pub info: T, // 吐给前端完美的、组装好的单体 VO
}

/// # [RESPONSE] - 列表数据响应
#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
    pub list: Vec<T>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 构造
impl<T: Serialize> ListResponse<T> {
    // 💡

    ////////

    /// # 1. [CASE] - 空列表
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(),
        }
    }

    /// # 2. [CASE] - 从Vec构造
    pub fn new(list: Vec<T>, page_info: PageInfo) -> Self {
        Self { list, page_info }
    }
}

////////

impl<T: Serialize> Default for ListResponse<T> {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
