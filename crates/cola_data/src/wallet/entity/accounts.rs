// cola_data/src/wallet/entity/accounts.rs
// ✅ WALLET - 钱包多资产账户实体
// 2026/8/20  Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

////////

/// # [ENTITY] - 钱包多资产账户表
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table name`: `wallet_accounts` - 表名
/// * `condition`: `金额使用 Decimal；积分等整数资产由业务层限制为整数`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletAccount {
    pub id: i64,                                 // 账户ID
    pub user_id: i64,                            // 用户ID（关联 cola_user.user.id）
    pub currency_id: i16,                        // 资产/货币ID（关联 wallet_currencies.id）
    pub balance: Decimal,                        // 可用余额
    pub frozen_balance: Decimal,                 // 冻结余额
    pub total_income: Decimal,                   // 累计收入
    pub total_expense: Decimal,                  // 累计支出
    pub pay_password: Option<String>,            // 支付密码哈希
    pub pay_password_set: bool,                  // 是否设置支付密码
    pub status: i16,                             // 状态：0-冻结 1-正常 2-注销
    pub freeze_reason: Option<String>,           // 冻结原因
    pub daily_limit: Option<Decimal>,            // 单日限额
    pub single_limit: Option<Decimal>,           // 单笔限额
    pub version: i64,                            // 乐观锁版本号
    pub last_transfer_at: Option<DateTime<Utc>>, // 最近一次转账时间
    pub created_at: DateTime<Utc>,               // 创建时间
    pub updated_at: DateTime<Utc>,               // 更新时间
}

//////// END
