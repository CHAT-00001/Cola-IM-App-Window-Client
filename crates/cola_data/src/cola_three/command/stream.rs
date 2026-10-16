// cola_data/src/cola_three/cmd/stream.rs  -- THREE - 直播推流配置命令
// 2026/6/30 03:47

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 新增/更新直播推流配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertStreamConfigCommand {
    pub id: Option<i64>,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub push_domain: String,
    pub access_key: String,
    pub secret_key: String,
    pub expire_seconds: i32,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

impl UpsertStreamConfigCommand {
    //

    ////////

    /// # 1. [BUILD] - 用于创建新记录的构造方法
    pub fn new(
        type_id: i64,
        vendor_id: i64,
        name: String,
        push_domain: String,
        access_key: String,
        secret_key: String,
        expire_seconds: i32,
    ) -> Self {
        Self {
            id: None,
            type_id,
            vendor_id,
            name,
            push_domain,
            access_key,
            secret_key,
            expire_seconds,
            config_json: None,
            remark: None,
            status: 1, // 默认启用
        }
    }

    ////////

    /// # 2. [BUILD] - 更新
    pub fn update(
        id: i64,
        type_id: i64,
        vendor_id: i64,
        name: String,
        push_domain: String,
        access_key: String,
        secret_key: String,
        expire_seconds: i32,
    ) -> Self {
        Self {
            id: Some(id),
            type_id,
            vendor_id,
            name,
            push_domain,
            access_key,
            secret_key,
            expire_seconds,
            config_json: None,
            remark: None,
            status: 1,
        }
    }
}

//////// END
