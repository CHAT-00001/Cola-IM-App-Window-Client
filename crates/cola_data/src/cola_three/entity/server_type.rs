// cola_data/src/cola_three/entity/server_type.rs  -- THREE - entity - 服务类型
// 2026/6/18 14:02

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方服务类型
/// * `pg schema`: `cola_three`
/// * `table name`: `server_type`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeServerTypeEntity {
    pub id: i64,                           // ID
    pub uid: i64,                          // 作者 ID
    pub code: String,                      // node / sms / stream / cola_im / email / pay
    pub name: String,                      // 英文名
    pub name_zh: String,                   // 中文名
    pub icon: Option<String>,              // 图标
    pub thumb: Option<String>,             // 缩略图
    pub sort: i16,                         // 排序
    pub status: i16,                       // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

////////

/// # 查询字段常量
pub const THREE_SERVER_TYPE_COLUMNS: &str = r#"
    id, uid, app, name, name_zh, icon, thumb, sort, status, created_at, updated_at
"#;

//////// END
