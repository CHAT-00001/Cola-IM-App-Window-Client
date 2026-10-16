// cola_data/src/cola_three/entity/stream.rs  -- -- THREE - 直播推流服务配置实体
// 2026/6/30 03:14

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方直播推流服务配置
/// * `table name`: `three_stream`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeStreamEntity {
    pub id: i64,
    pub type_id: i64,                           // FK → three_type.id
    pub vendor_id: i64,                         // FK → three_vendor.id
    pub name: String,                           // 服务名称，如"腾讯云直播"
    pub push_domain: String,                    // 推流域名 (RTMP/SRT)
    pub access_key: String,                     // 鉴权访问密钥/AppKey
    pub secret_key: String,                     // 鉴权签名密钥/AppSecret（加密存储）
    pub expire_seconds: i32,                    // 推流地址过期时间（秒）
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（如AppName、StreamID规则、SRT/RTMP切换）
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,      // 创建时间
    pub updated_at: Option<DateTime<Utc>>,      // 修改时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const STREAM_CONFIG_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, push_domain, access_key, secret_key,
    expire_seconds, config_json, remark, status, created_at, updated_at
"#;

//////// END
