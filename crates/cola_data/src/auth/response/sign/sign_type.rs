// cola_data/src/code/response/sign/sign_type.rs  -- 数据中心 - AUTH - response - sign - 类型
// 2026/7/28 10:39

////////

use crate::app::page::PageInfo;
use crate::auth::info::sign_type::SignTypeInfo;
use serde::Serialize;

////////

/// # [RESPONSE] - 单视频响应
#[derive(Debug, Serialize)]
pub struct SignTypeSingleResponse {
    pub info: SignTypeInfo, // 吐给前端完美的、组装好的 Info 列表
}

////////

/// # [RESPONSE] - 多视频响应
#[derive(Debug, Serialize)]
pub struct SignTypeListResponse {
    pub list: Vec<SignTypeInfo>, // 吐给前端完美的、组装好的 Info 列表
    pub page_info: PageInfo,
}

// 构造实现
impl SignTypeListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for SignTypeListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
