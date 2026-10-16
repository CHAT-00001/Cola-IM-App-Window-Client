// cola_data/src/dynamic/cmd/hotlist.rs -- 数据 - 动态 - cmd - 上热门 - mod
// 2026/5/22 20:45 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 动态 - 上热门命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamicHotlistCommand {
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub dynamic_id: i64,        // 动态 ID
    pub remark: Option<String>, // 备注
    pub qty: i32,               // 曝光量
    pub time_zone: String,      // 时间区间
    pub older: Option<String>,  // 订单 ID
}
