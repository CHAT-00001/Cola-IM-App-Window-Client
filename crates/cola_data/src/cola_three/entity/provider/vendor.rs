// cola_data/src/cola_three/entity/vendor  -- THREE - 厂商实体
// 2026/6/18

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方厂商名称
/// * `pg schema`: `cola_three`
/// * `table name`: `three_vendor`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeVendorEntity {
    pub id: i64,                           // ID
    pub _id: String,                       // 备用ID
    pub code: String,                      // 三字母代码
    pub name: String,                      // 英文名称
    pub name_zh: String,                   // 中文名
    pub remark: Option<String>,            // 备注(可选)
    pub sort: i16,                         // 排序: 默认9999
    pub status: i16,                       // 1启用 0禁用
    pub owner: i16,                        // 所有权: 0. 私有 1. 第三方
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const THREE_VENDOR_COLUMNS: &str = r#"
    id, app, name, name_zh, remark, sort, status, owner, add_time, upd_time, created_at, updated_at
"#;

//////// END
