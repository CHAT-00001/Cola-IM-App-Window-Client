// cola_data/src/wallet/entity/transactions.rs
// ✅ WALLET - 资产交易流水实体
// 2026/8/20  Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

////////

/// # [ENTITY] - 资产交易流水表
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table name`: `wallet_transactions`
/// * `condition`: `金额使用 Decimal；一笔业务可能产生多条不同账户流水`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletTransaction {
    pub id: i64,                             // 流水ID
    pub tx_no: String,                       // 交易编号（业务唯一标识）
    pub user_id: i64,                        // 用户ID
    pub wallet_id: i64,                      // 钱包账户ID
    pub currency_id: i16,                    // 资产/货币ID
    pub tx_type: String, // 交易类型：RECHARGE-充值, WITHDRAW-提现, TRANSFER-转账, PAY-支付, REFUND-退款, COIN_EARN-赚金币, COIN_SPEND-花金币, ADMIN-后台调整
    pub tx_direction: String, // 方向：IN-收入, OUT-支出
    pub amount: Decimal, // 交易金额（正数）
    pub balance_before: Decimal, // 交易前余额
    pub balance_after: Decimal, // 交易后余额
    pub fee: Decimal,    // 手续费
    pub fee_currency_id: Option<i16>, // 手续费资产ID
    pub status: i16,     // 状态：0-处理中 1-成功 2-失败 3-已取消 4-退款中
    pub fail_reason: Option<String>, // 失败原因
    pub channel: Option<String>, // 渠道：WECHAT-微信, ALIPAY-支付宝, BANK-银行卡, SYSTEM-系统, COIN-金币
    pub channel_tx_no: Option<String>, // 渠道交易号（第三方流水号）
    pub idempotency_key: Option<String>, // 业务幂等键
    pub business_type: Option<String>, // 业务类型
    pub business_id: Option<String>, // 业务单号
    pub remark: Option<String>,  // 备注
    pub extra: JsonValue,        // 扩展信息（JSON）
    pub source: Option<String>,  // 来源：ANDROID, IOS, WEB, ADMIN, SYSTEM
    pub source_ip: Option<String>, // 来源IP
    pub completed_at: Option<DateTime<Utc>>, // 完成时间
    pub created_at: DateTime<Utc>, // 创建时间
    pub updated_at: DateTime<Utc>, // 更新时间
}

//////// END
