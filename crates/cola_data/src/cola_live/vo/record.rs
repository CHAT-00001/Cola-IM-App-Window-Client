// cola_data/src/cola_live/vo/record.rs
// 数据 - LIVE - vo - 直播场次列表
// 2026/8/21 09:30 Created.

////////

use crate::app::page::PageInfo;
use crate::cola_live::info::record::LiveRecordInfo;
use serde::{Deserialize, Serialize};

////////

/// # 1. [VO] - 直播场次视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRecordVo {
    pub info: LiveRecordInfo, // 直播场次信息
}

impl From<LiveRecordInfo> for LiveRecordVo {
    fn from(info: LiveRecordInfo) -> Self {
        Self { info }
    }
}

////////

/// # 2. [VO] - 直播场次列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRecordListVo {
    pub list: Vec<LiveRecordVo>, // 直播列表
    pub page_info: PageInfo,     // 分页信息
}

//////// END
