// cola_data/src/cola_coc/cmd/buy.rs -- 数据 - COC - cmd - 购买命令 - mod
// 2026/5/22 20:51 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 内容运营中心 -  购买命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuyCommand {
    pub user_id: i64,           // 用户 ID
    pub app_id: i16,            // 应用 ID
    pub content_id: i64,        // 内容 ID
    pub remark: Option<String>, // 备注
    pub qty: i32,               // 数量
    pub order_id: Option<i64>,  // 订单 ID
}

//////// END
