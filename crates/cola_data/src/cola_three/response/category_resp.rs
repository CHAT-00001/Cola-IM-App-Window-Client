// cola_data/src/cola_three/response/category_resp.rs  -- 数据中心 - 第三方 - 响应体 - 分类响应
// 2026/7/27 13:27

////////

use crate::app::page::PageInfo;
use crate::cola_three::vo::category::CategoryVo;
use serde::Serialize;

////////

/// # [RESPONSE] - 单个响应
#[derive(Debug, Serialize)]
pub struct CateSingleResponse {
    pub info: CategoryVo, // 分类视图模型
}

/// # [RESPONSE] - 列表响应
#[derive(Debug, Serialize)]
pub struct CateListResponse {
    pub list: Vec<CategoryVo>, // 分类视图模型
    pub page_info: PageInfo,   // 分页
}

// 构造实现
impl CateListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for CateListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
