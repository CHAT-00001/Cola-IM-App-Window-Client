// cola_data/src/cola_three/entity/binding  -- THREE - 业务绑定实体
// 2026/6/18

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//////

/// # [ENTITY] - 第三方业务绑定
/// * table name: three_biz_binding
/// * UNIQUE (biz_module, biz_type) — 每个模块每种服务只能绑一个
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeBizBindingEntity {
    pub id: i64,
    pub three_config_id: i64, // FK → three_config.id
    pub biz_module: String,   // video / user / cola_live / music
    pub biz_type: String,     // node / sms / stream
    pub status: i16,          // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// # 查询字段常量
pub const THREE_BIZ_BINDING_COLUMNS: &str = r#"
    id, three_config_id, biz_module, biz_type, status, created_at, updated_at
"#;
