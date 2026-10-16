// cola_data/src/wallet/entity/currency.rs
// ✅ WALLET - 资产/货币定义实体
// 2026/8/20  Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

////////

/// # [ENTITY] - 资产/货币种类表
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table name`: `wallet_currencies`
/// * `desc`: `POINT、COIN、CNY、USD 等资产统一由 app 标识；usd_rate 仅用于展示或汇率业务，不参与余额存储`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletCurrency {
    pub id: i16,                   // 货币ID
    pub code: String,              // 货币代码：USD, CNY, JPY, KRW, HKD, GBP, SGD, EUR, COIN, POINT
    pub name: String,              // 货币名称
    pub name_en: String,           // 英文名称
    pub symbol: String,            // 货币符号
    pub symbol_native: String,     // 本地货币符号
    pub decimal_places: i16,       // 小数位数（USD/CNY=2, JPY=0, COIN=0）
    pub usd_rate: Decimal,         // 对美元汇率（1 USD = ? 该货币）
    pub is_base: bool,             // 是否是基准货币（USD）
    pub is_crypto: bool,           // 是否是虚拟货币
    pub is_point: bool,            // 是否是积分
    pub is_enabled: bool,          // 是否启用
    pub sort_order: i16,           // 排序权重
    pub icon_url: Option<String>,  // 图标URL
    pub extra: JsonValue,          // 扩展信息 (JSONB)
    pub created_at: DateTime<Utc>, // 创建时间
    pub updated_at: DateTime<Utc>, // 更新时间
}
