// cola_data/src/cola_three/cmd/pay.rs  -- THREE - 第三方支付配置命令
// 2026/6/30 04:01

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 新增/更新第三方支付配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertPayConfigCommand {
    pub id: Option<i64>,                        // ID
    pub type_id: i64,                           // 类型ID
    pub vendor_id: i64,                         // 厂商
    pub name: String,                           // 名称
    pub mch_id: String,                         // 商户号 (Merchant ID)
    pub api_key: String,                        // API 密钥 (用于签名)
    pub notify_url: String,                     // 支付回调通知地址
    pub config_json: Option<serde_json::Value>, // 核心扩展：存放证书路径、公私钥、支付环境(沙箱/正式)
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 状态
}

impl UpsertPayConfigCommand {
    //

    ////////

    /// # 1. [BUILD] - 新建
    pub fn new(
        type_id: i64,
        vendor_id: i64,
        name: String,
        mch_id: String,
        api_key: String,
        notify_url: String,
    ) -> Self {
        Self {
            id: None,
            type_id,
            vendor_id,
            name,
            mch_id,
            api_key,
            notify_url,
            config_json: None,
            remark: None,
            status: 1, // 默认启用
        }
    }

    /// # 2. [BUILD] - 更新
    pub fn update(
        id: i64,
        type_id: i64,
        vendor_id: i64,
        name: String,
        mch_id: String,
        api_key: String,
        notify_url: String,
    ) -> Self {
        Self {
            id: Some(id),
            type_id,
            vendor_id,
            name,
            mch_id,
            api_key,
            notify_url,
            config_json: None,
            remark: None,
            status: 1,
        }
    }
}

//////// END
