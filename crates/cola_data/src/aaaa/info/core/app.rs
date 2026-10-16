// cola_data/src/aaaa/info/core/app.rs -- DATA - BASIC - info - 核心 - 应用信息
// 2026/8/9 07:38 Created.

////////

use crate::aaaa::entity::core::app::AppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 基础 - 核心应用信息
/// * `desc`: `平台应用的安全信息`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: i64,                           // ID
    pub _id: Option<String>,               // UUID v4
    pub app_id: Option<String>,            // 应用 ID
    pub type_id: i64,                      // 类型 ID
    pub vendor_id: i64,                    // 厂商 ID
    pub app_code: String,                  // 应用代码
    pub app_name: String,                  // 应用名称
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
impl From<AppEntity> for AppInfo {
    //

    ////////

    /// # [FROM] - 转换
    /// * `desc`: `元信息来源于entity表`
    fn from(e: AppEntity) -> Self {
        Self {
            id: e.id,
            _id: e._id,
            app_id: e.app_id,
            type_id: e.type_id,
            vendor_id: e.vendor_id,
            app_code: e.app_code,
            app_name: e.app_name,
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
