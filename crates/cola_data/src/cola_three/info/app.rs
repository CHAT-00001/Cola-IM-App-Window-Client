// cola_data/src/cola_three/info/app.rs
// 数据 - THREE - info - 应用模块信息
// 2026/8/15 12:45 Created.

////////

use crate::cola_three::entity::app::ColaAppEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 应用模块安全缓存信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppInfo {
    pub id: i64,
    pub app_id: String,
    pub name: String,
    pub status: i16,
}

impl From<ColaAppEntity> for AppInfo {
    fn from(e: ColaAppEntity) -> Self {
        Self {
            id: e.id,
            app_id: e.app_id,
            name: e.name,
            status: e.status,
        }
    }
}

//////// END
