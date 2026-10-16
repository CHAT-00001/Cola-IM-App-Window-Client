// cola_data/src/aaaa/cmd/node  -- 数据 - FS - cmd - CDN命令载荷
// 2026/7/27 14:40

////////

use serde::{Deserialize, Serialize};

////////

/// # [CMD] - 节点创建载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCreateCmd {
    pub _id: Option<String>,
    pub app_id: Option<String>,
    pub bucket_key: String,
    pub cdn_domain: String,
    pub provider: i16,
    pub is_https: bool,
    pub auth_type: i16,
    pub auth_key: Option<String>,
}

/// # [CMD] - 节点更新命令载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeUpdateCmd {
    pub cdn_domain: Option<String>,
    pub provider: Option<i16>,
    pub is_https: Option<bool>,
    pub is_enabled: Option<bool>,
    pub auth_type: Option<i16>,
    pub auth_key: Option<String>,
    pub status: Option<i16>,
}

//////// END
