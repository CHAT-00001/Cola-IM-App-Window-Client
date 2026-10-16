// cola_data/src/wallet/entity/pay.rs
// ✅ WALLET - 第三方支付渠道配置实体（兼容 three_pay）
// 2026/8/20  Updated.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方支付服务配置
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table name`: `three_pay`（历史表名，后续可迁移至 cola_wallet.pay）
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreePayConfigEntity {
    pub id: i64,
    pub type_id: i64,                           // FK → three_type.id
    pub vendor_id: i64,                         // FK → three_vendor.id
    pub name: String,                           // 服务名称，如"微信支付"、"支付宝"
    pub mch_id: String,                         // 商户号 (Merchant ID)
    pub api_key: String,                        // API 密钥 (用于签名，加密存储)
    pub notify_url: String,                     // 支付回调通知地址
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（证书内容、沙箱模式开关、应用公钥等）
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,      // 创建时间
    pub updated_at: Option<DateTime<Utc>>,      // 更新时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const THREE_PAY_CONFIG_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, mch_id, api_key, notify_url,
    config_json, remark, status, created_at, updated_at
"#;

//////// END
