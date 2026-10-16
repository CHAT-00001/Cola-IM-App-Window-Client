// cola_data/src/cola_three/entity//sms_config.rs  --  -- THREE - 服务配置实体
// 2026/6/30 03:03

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方短信服务配置
/// * `table name`: `three_sms`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeSmsConfigEntity {
    pub id: i64,
    pub type_id: i64,                           // FK → three_type.id
    pub vendor_id: i64,                         // FK → three_vendor.id
    pub name: String,                           // 服务名称，如"阿里云短信"
    pub access_key: String,                     // 访问密钥/AppKey
    pub secret_key: String,                     // 密钥/AppSecret（加密存储）
    pub endpoint: String,                       // 接入端点/API地址
    pub region: String,                         // 区域（部分厂商需要，如 cn-hangzhou）
    pub sign_name: String,                      // 短信签名
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（如模板ID映射、通道类型等）
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,      // 创建时间
    pub updated_at: Option<DateTime<Utc>>,      // 更新时间
}

////////

/// # 查询字段常量
pub const THREE_SMS_CONFIG_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, access_key, secret_key,
    endpoint, region, sign_name, config_json, remark, status, created_at, updated_at
"#;

//////// END
