// cola_data/src/wallet/port/point.rs
// ✅ WALLET - 积分账户初始化端口数据
// 2026/8/20 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [PORT DATA] - 积分账户初始化结果
/// * `desc`: `返回积分账户和首笔初始化交易的关键结果`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPointInitResult {
    pub account_id: i64,             // 积分账户ID
    pub user_id: i64,                // 用户ID
    pub balance: i64,                // 当前积分余额（积分最小单位为1）
    pub transaction_id: Option<i64>, // 首笔赠送交易ID
    pub is_new_account: bool,        // 是否本次创建账户
}

//////// END
