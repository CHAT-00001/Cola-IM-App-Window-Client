// cola_data/src/app/page.rs -- DATA - APP - 分页响应
// 2026/5/21 02:28 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [RESPONSE] - 分页信息
/// * 前台
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageInfo {
    pub page: i64,      // 当前页码
    pub qty: i64,       // 每页数量
    pub has_more: bool, // 是否还有下一页
}

// 🌟 手动实现 Default 特征，自定义空数据时的初始值 🌟
impl Default for PageInfo {
    fn default() -> Self {
        Self {
            page: 1,         // 默认起始页码为 1
            qty: 10,         // 默认每页 10 条
            has_more: false, // 默认没有下一页
        }
    }
}

/// # [RESPONSE] - 分页信息
/// * 管理员
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPageInfo {
    pub page: i64,      // 当前页码
    pub qty: i64,       // 每页数量
    pub total: i64,     // 总记录数（管理员审计才需要）
    pub has_more: bool, // 是否还有下一页
}

/// # [RESPONSE] - 列表响应 (保持泛型)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub list: Vec<T>,
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub qty: Option<i64>,
    pub total: Option<i64>,
    pub has_more: Option<bool>,
}

//////// END
