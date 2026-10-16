// cola_data/src/entity/report/classify.rs -- 数据 - COC - entity - 举报 - 分类表
// 2026/9/1 04:59 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 运营中心 - 举报分类表
/// * `pg schema`: `cola_coc` - PG 模式
/// * `table name`: `report_classify` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CocReportClassify {
    pub id: i16,                           // 举报类型 ID
    pub list_order: i32,                   // 列表排序
    pub name: String,                      // 举报类型名称
    pub name_zh: Option<String>,           // 中文名称
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 举报分类表字段常量定义
/// * `desc`: `严格与 CocReportClassify 结构体字段顺序和名称对齐`
pub const REPORT_CLASSIFY_COLUMNS: &str = "\
    id, list_order, name, name_zh, status, \
    is_deleted, created_at, updated_at, deleted_at\
    ";

//////// END
