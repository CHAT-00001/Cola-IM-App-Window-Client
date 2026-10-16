// cola_data/src/cola_three/cmd/sign_type  -- THREE - 第三方登录配置命令
// 2026/6/30 03:55

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 新增/更新第三方登录配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertThreeSignCommand {
    pub id: Option<i64>,                        // ID
    pub type_id: i64,                           // 类型ID
    pub vendor_id: i64,                         // 厂商 ID
    pub name: String,                           // 名称
    pub client_id: String,                      // 客户端ID
    pub client_secret: String,                  // 客户端秘钥
    pub redirect_uri: String,                   // 重定向 URI
    pub scope: String,                          // 权限范围
    pub config_json: Option<serde_json::Value>, // 配置JSON
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 状态: 0禁用 1启用
}

impl UpsertThreeSignCommand {
    //

    ////////

    /// # 1. [BUILD] - 新建
    pub fn new(
        type_id: i64,
        vendor_id: i64,
        name: String,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scope: String,
    ) -> Self {
        Self {
            id: None,
            type_id,
            vendor_id,
            name,
            client_id,
            client_secret,
            redirect_uri,
            scope,
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
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scope: String,
    ) -> Self {
        Self {
            id: Some(id),
            type_id,
            vendor_id,
            name,
            client_id,
            client_secret,
            redirect_uri,
            scope,
            config_json: None,
            remark: None,
            status: 1,
        }
    }
}

//////// END
