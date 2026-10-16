// cola_data/src/cola_three/entity/app.rs
// 数据 - THREE - entity - 应用模块
// 2026/8/15 12:45 Created.

////////

use crate::cola_three::info::app::AppInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方/基础大模块 - 应用模块
/// * `pg schema`: `cola_three`
/// * `table name`: `cola_app`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ColaAppEntity {
    pub id: i64,
    pub app_id: String,              // 业务唯一标识，如 "short_video", "cola_live"
    pub name: String,                // 业务中文名，如 "可乐短视频"
    pub description: Option<String>, // 描述
    pub status: i16,                 // 状态：1. 启用 0. 禁用
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

////////

/// # [COLUMNS] - 查询字段常量
pub const COLA_APP_COLUMNS: &str = r#"
    id, app_id, name, description, status, created_at, updated_at
"#;

impl ColaAppEntity {
    /// # 1. [ENTITY] - 转换为安全缓存信息
    pub fn to_app_info(&self) -> anyhow::Result<AppInfo> {
        Ok(AppInfo {
            id: self.id,
            app_id: self.app_id.clone(),
            name: self.name.clone(),
            status: self.status,
        })
    }
}

//////// END
