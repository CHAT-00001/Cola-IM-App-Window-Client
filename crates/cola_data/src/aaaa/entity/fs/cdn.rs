// cola_data/src/fs/entity/node  -- 数据 - FS - entity - CDN加速配置
// 2026/8/14 13:00

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 文件服务 - CDN 加速配置实体
/// * `pg schema`: `cola_fs` -- PG 模式
/// * `table name`: `cdn_domain` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CdnDomainEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub app_id: Option<String>,            // 所属应用/模块标识
    pub bucket_key: String,                // 关联的逻辑存储桶编码
    pub cdn_domain: String,                // CDN 加速域名 (如: https://cdn.example.com)
    pub provider: i16, // CDN 服务商: 1-阿里云CDN, 2-腾讯云CDN, 3-Cloudflare, 4-自建Nginx
    pub is_https: bool, // 是否启用 HTTPS
    pub is_enabled: bool, // 是否启用加速
    pub auth_type: i16, // 鉴权类型: 0-无鉴权(公开), 1-URL鉴权A, 2-URL鉴权B
    pub auth_key: Option<String>, // CDN 边缘鉴权密钥 (防盗链签名 Key)
    pub status: i16,   // 状态码: 1-正常, 0-停用
    pub is_deleted: Option<bool>, // 逻辑删除
    pub created_time: i64, // 创建时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

/// # 2.[COLUMNS] - 数据表原始字段
pub const CDN_DOMAIN_COLUMNS: &str = r#"
    id, _id, app_id, bucket_key, cdn_domain, provider,
    is_https, is_enabled, auth_type, auth_key, status,
    is_deleted, created_time, created_at, updated_at, deleted_at
"#;

//////// END
