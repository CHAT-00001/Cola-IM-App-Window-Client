// cola_data/src/cola_three/cmd/cola_fs.rs -- THREE - 配置命令
// 2026/6/30 03:47

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 新增/更新文件存储配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertFileStorageCommand {
    pub id: Option<i64>,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

impl UpsertFileStorageCommand {
    //

    ////////

    /// # 1. [BUILD] - 新建
    pub fn new(
        type_id: i64,
        vendor_id: i64,
        name: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        endpoint: String,
        region: String,
    ) -> Self {
        Self {
            id: None,
            type_id,
            vendor_id,
            name,
            bucket,
            access_key,
            secret_key,
            endpoint,
            region,
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
        bucket: String,
        access_key: String,
        secret_key: String,
        endpoint: String,
        region: String,
    ) -> Self {
        Self {
            id: Some(id),
            type_id,
            vendor_id,
            name,
            bucket,
            access_key,
            secret_key,
            endpoint,
            region,
            config_json: None,
            remark: None,
            status: 1,
        }
    }
}

//////// END
