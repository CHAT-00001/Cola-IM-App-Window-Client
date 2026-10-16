// data/src/aaaa/info/node -- DATA - AAAA - INFO - 核心 - 域名信息
// 2026/8/9 07:38 Created.

////////

use crate::cola_fs::entity::cdn::CdnDomainEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 基础 - 节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub app_id: Option<String>,
    pub bucket_key: String,
    pub cdn_domain: String,
    pub provider: i16,
    pub is_https: bool,
    pub is_enabled: bool,
    pub auth_type: i16,
    pub status: i16,
    pub created_at: DateTime<Utc>,
}

// 构造实现
impl From<CdnDomainEntity> for NodeInfo {
    fn from(entity: CdnDomainEntity) -> Self {
        Self {
            id: entity._id.unwrap_or_else(|| entity.id.to_string()),
            app_id: entity.app_id,
            bucket_key: entity.bucket_key,
            cdn_domain: entity.cdn_domain,
            provider: entity.provider,
            is_https: entity.is_https,
            is_enabled: entity.is_enabled,
            auth_type: entity.auth_type,
            status: entity.status,
            created_at: entity.created_at.unwrap_or_else(Utc::now),
        }
    }
}

//////// END
