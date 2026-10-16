// cola_data/src/coc/cmd/hotlist/record.rs -- 数据 - COC - cmd - 上热门 - 记录命令
// 2026/9/1 05:24 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 内容运营 - 上热门命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HotlistCommand {
    pub user_id: i64,           // 用户 ID
    pub app_id: i16,            // 应用 ID
    pub content_id: i64,        // 内容 ID
    pub remark: Option<String>, // 备注(管理员操作时可选)
    pub exposure_qty: i32,      // 曝光量
    pub order_id: Option<i64>,  // 订单 ID
}
