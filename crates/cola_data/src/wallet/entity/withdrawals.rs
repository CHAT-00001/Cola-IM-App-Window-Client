// cola_data/src/wallet/entity/withdrawals.rs
// ✅ WALLET - 提现订单实体
// 2026/8/20  Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};

////////

/// # [ENTITY] - 提现记录表
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table_name`: `wallet_withdrawals`
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WalletWithdrawal {
    pub id: i64,                             // BIGSERIAL 主键
    pub tx_no: String,                       // 交易编号
    pub user_id: i64,                        // 用户ID
    pub currency_id: i16,                    // 提现资产ID
    pub wallet_id: i64,                      // 钱包ID
    pub amount: Decimal,                     // 提现金额
    pub fee: Decimal,                        // 手续费
    pub actual_amount: Decimal,              // 实际到账金额
    pub status: i16,                         // 状态：0-待审核 1-处理中 2-成功 3-失败 4-已取消
    pub bank_name: Option<String>,           // 银行名称
    pub bank_card_no: Option<String>,        // 银行卡号（脱敏）
    pub bank_card_owner: Option<String>,     // 持卡人姓名
    pub bank_branch: Option<String>,         // 支行信息
    pub alipay_account: Option<String>,      // 支付宝账号
    pub wechat_account: Option<String>,      // 微信账号
    pub withdraw_method: String,             // 提现方式：BANK, ALIPAY, WECHAT
    pub channel_tx_no: Option<String>,       // 渠道交易号
    pub fail_reason: Option<String>,         // 失败原因
    pub auditor_id: Option<i64>,             // 审核人ID
    pub audited_at: Option<DateTime<Utc>>,   // 审核时间
    pub completed_at: Option<DateTime<Utc>>, // 完成时间
    pub remark: Option<String>,              // 备注
    pub extra: JsonValue,                    // 扩展信息（JSONB）
    pub created_at: DateTime<Utc>,           // 创建时间
    pub updated_at: DateTime<Utc>,           // 更新时间
}
