// cola_data/src/aaaa/cmd/core/app.rs -- DATA - BASIC - cmd - 核心 - 应用
// 2026/7/27 14:39

////////

use serde::{Deserialize, Serialize};
use serde_json::Value;

////////

/// # [CMD] - 基础 - 应用创建命令
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppCreateCmd {
    pub _id: Option<String>,        // UUID v4 无分隔符副 ID
    pub app_id: Option<String>,     // 应用 ID
    pub type_id: i64,               // 存储类型 ID
    pub vendor_id: i64,             // 厂商 ID
    pub app_code: String,           // 应用代码
    pub app_name: String,           // 应用名称
    pub bucket: String,             // 厂商真实存储桶
    pub cdn_domain: Option<String>, // CDN 加速域名
    pub access_key: String,         // 访问密钥
    pub secret_key: String,         // 私密密钥
    pub endpoint: String,           // 接入端点
    pub region: String,             // 区域
    pub config_json: Option<Value>, // 厂商扩展配置
    pub remark: Option<String>,     // 备注
    pub status: i16,                // 状态码

    #[serde(default)]
    pub is_public: bool, // 是否公开 (默认 false)

    #[serde(default)]
    pub is_deleted: bool, // 逻辑删除 (默认 false)
}

////////

impl AppCreateCmd {
    //

    ////////

    /// # 1. [CMD] - 创建存储桶命令构造器
    /// * `desc`: 自动生成无分隔符 UUID，名称为空时使用默认名称，其余存储配置由客户端提交
    pub fn new(
        app_id: Option<String>,     // 应用 ID
        type_id: i64,               // 存储类型 ID
        vendor_id: i64,             // 厂商 ID
        app_code: Option<String>,       // 应用代码
        app_name: Option<String>,       // 应用名称
        bucket: String,             // 厂商真实存储桶
        cdn_domain: Option<String>, // CDN 域名
        access_key: String,         // 访问密钥
        secret_key: String,         // 私密密钥
        endpoint: String,           // 接入端点
        region: String,             // 区域
        config_json: Option<Value>, // 厂商扩展配置
        remark: Option<String>,     // 备注
        status: i16,                // 状态码
        is_public: Option<bool>,    // 是否公开
        is_deleted: Option<bool>,   // 逻辑删除
    ) -> Self {
        Self {
            _id: Some(uuid::Uuid::new_v4().simple().to_string()),
            app_id,
            type_id,
            vendor_id,
            app_code: app_code
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "COLA-0001".to_string()),
            app_name: app_name
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "默认应用".to_string()),
            bucket,
            cdn_domain,
            access_key,
            secret_key,
            endpoint,
            region,
            config_json,
            remark,
            status,
            is_public: is_public.unwrap_or(false), // 默认值 false
            is_deleted: is_deleted.unwrap_or(false), // 默认值 false
        }
    }

    ////////

    /// # 2. [CMD] - 补全创建命令默认值
    /// * `desc`: 客户端反序列化后调用，补全空名称
    pub fn complete_defaults(&mut self) {
        if self._id.as_deref().unwrap_or_default().is_empty() {
            self._id = Some(uuid::Uuid::new_v4().simple().to_string());
        }
        if self.app_name.trim().is_empty() {
            self.app_name = "默认应用".to_string();
        }
    }
}

////////

/// # [CMD] - 应用更新命令载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpdateCmd {
    pub app_id: Option<String>,
    pub app_code: Option<String>,
    pub app_name: Option<String>,
    pub cdn_domain: Option<String>,
    pub type_id: Option<i64>,
    pub vendor_id: Option<i64>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub config_json: Option<Value>,
    pub remark: Option<String>,
    pub is_public: Option<bool>,
    pub is_banned: Option<bool>,
    pub is_deleted: Option<bool>, // 逻辑删除更新
    pub status: Option<i16>,
}

//////// END
