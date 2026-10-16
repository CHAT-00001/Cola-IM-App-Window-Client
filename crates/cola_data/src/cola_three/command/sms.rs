// cola_data/src/cola_three/cmd/sms.rs  -- THREE - 短信配置命令
// 2026/6/30 03:47

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 新增/更新短信服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertSmsConfigCommand {
    pub id: Option<i64>,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub sign_name: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

impl UpsertSmsConfigCommand {
    /// # 用于创建新记录的构造方法
    pub fn new(
        type_id: i64,
        vendor_id: i64,
        name: String,
        access_key: String,
        secret_key: String,
        endpoint: String,
        region: String,
        sign_name: String,
    ) -> Self {
        Self {
            id: None,
            type_id,
            vendor_id,
            name,
            access_key,
            secret_key,
            endpoint,
            region,
            sign_name,
            config_json: None,
            remark: None,
            status: 1, // 默认启用
        }
    }

    /// # 用于更新现有记录的构造方法
    pub fn update(
        id: i64,
        type_id: i64,
        vendor_id: i64,
        name: String,
        access_key: String,
        secret_key: String,
        endpoint: String,
        region: String,
        sign_name: String,
    ) -> Self {
        Self {
            id: Some(id),
            type_id,
            vendor_id,
            name,
            access_key,
            secret_key,
            endpoint,
            region,
            sign_name,
            config_json: None,
            remark: None,
            status: 1,
        }
    }
}
