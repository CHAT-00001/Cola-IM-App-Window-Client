// data/src/cola_infra/info/fs/node -- DATA - INFRA - info - 文件存储 - CDN域名信息
// 2026/8/9 07:38 Created.

////////

use crate::cola_fs::entity::cdn::CdnDomainEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 文件服务 - CDN 配置安全脱敏视图（隐藏鉴权密钥）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnDomainInfo {
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
impl From<CdnDomainEntity> for CdnDomainInfo {
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
