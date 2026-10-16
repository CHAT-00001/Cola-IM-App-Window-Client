// data/src/cola_fs/info/bucket.rs
// 数据 - FS - info - 存储桶
// 2026/8/9 07:38 Created.

////////

use crate::cola_fs::entity::bucket::BucketEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 存储桶信息
/// * `desc`: `存储桶安全的信息`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketInfo {
    pub id: i64,                           // ID
    pub _id: Option<String>,               // UUID v4
    pub app_id: Option<String>,            // 应用 ID
    pub type_id: i64,                      // 类型 ID
    pub vendor_id: i64,                    // 厂商 ID
    pub name: String,                      // 名称
    pub bucket: String,                    // 存储桶名
    pub cdn_domain: Option<String>,        // CDN 域名
    pub endpoint: String,                  // 接入端点
    pub region: String,                    // 区域
    pub remark: Option<String>,            // 备注
    pub is_public: bool,                   // 是否公开
    pub is_banned: bool,                   // 是否禁用
    pub status: i16,                       // 状态
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

// 构造实现
impl From<BucketEntity> for BucketInfo {
    //

    ////////

    /// # [FROM] - 转换
    /// * `desc`: `元信息来源于entity表`
    fn from(e: BucketEntity) -> Self {
        Self {
            id: e.id,
            _id: e._id,
            app_id: e.app_id,
            type_id: e.type_id,
            vendor_id: e.vendor_id,
            name: e.name,
            bucket: e.bucket,
            cdn_domain: e.cdn_domain,
            endpoint: e.endpoint,
            region: e.region,
            remark: e.remark,
            is_public: e.is_public,
            is_banned: e.is_banned,
            status: e.status,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

//////// END
