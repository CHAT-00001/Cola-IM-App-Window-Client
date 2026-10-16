// cola_data/src/code/response/sign/sign_log.rs --
// 数据中心 - AUTH - response - sign - 登录日志
// 2026/7/28 10:50

////////

use crate::app::page::PageInfo;
use crate::auth::vo::sign_log::SignLogVo;
use serde::Serialize;
////////

/// # [RESPONSE] - 单记录响应
#[derive(Debug, Serialize)]
pub struct SignLogSingleResponse {
    pub info: SignLogVo, // 吐给前端完美的、组装好的 Info 列表
}

////////

/// # [RESPONSE] - 多记录响应
#[derive(Debug, Serialize)]
pub struct SignLogListResponse {
    pub list: Vec<SignLogVo>, // 吐给前端完美的、组装好的 Info 列表
    pub page_info: PageInfo,
}

// 构造实现
impl SignLogListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for SignLogListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
