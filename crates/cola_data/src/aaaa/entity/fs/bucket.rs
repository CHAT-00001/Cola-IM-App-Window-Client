// cola_data/src/cola_fs/entity/bucket.rs  -- 数据中心 - FS - entity - 存储桶
// 2026/7/27 14:38

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 文件存储 存储桶
/// * `pg schema`: `cola_fs`
/// * `table name`: `bucket`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct BucketEntity {
    pub id: i64,
    pub _id: Option<String>,                    // UUID v4
    pub app_id: Option<String>,                 // 应用 ID（绑定子业务模块）
    pub type_id: i64,                           // 类型 ID
    pub vendor_id: i64,                         // 厂商 ID
    pub name: String,                           // 服务名称，如"视频主CDN"
    pub cdn_domain: Option<String>,             // CDN 加速域名
    pub bucket: String,                         // 存储桶名
    pub access_key: String,                     // 访问密钥
    pub secret_key: String,                     // 密钥（加密存储）
    pub endpoint: String,                       // 接入端点
    pub region: String,                         // 区域
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（JSONB）
    pub remark: Option<String>,                 // 备注
    pub is_public: bool,                        // 是否公开桶
    pub is_banned: bool,                        // 是否禁用
    pub status: i16,                            // 1启用 0禁用
    pub is_deleted: bool,                       // 逻辑删除
    pub created_at: Option<DateTime<Utc>>,      // 创建时间
    pub updated_at: Option<DateTime<Utc>>,      // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,      // 删除时间
    pub reviewed_at: Option<DateTime<Utc>>,     // 审核时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const BUCKET_COLUMNS: &str = r#"
    id, _id, app_id, type_id, vendor_id, name, cdn_domain,
    bucket, access_key, secret_key, endpoint, region, config_json, remark,
    is_public, is_banned, status, is_deleted,
    created_at, updated_at, deleted_at, reviewed_at
"#;

//////// END
