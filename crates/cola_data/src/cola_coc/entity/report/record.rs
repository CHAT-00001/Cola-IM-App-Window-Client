// cola_data/src/cola_coc/entity/report/record.rs -- 数据 - COC - entity - 举报 - 举报记录表
// 2026/9/1 05:05 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 运营中心 - 举报记录表
/// * `pg schema`: `cola_coc` -- PG模式
/// * `table name`: `report_record` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CocReportEntity {
    pub id: i64,                           // 检举 ID
    pub uid: i64,                          // 用户 ID (举报人)
    pub app_id: i16,                       // 应用 ID
    pub content_id: i64,                   // 被举报的内容 ID
    pub classify_id: i16,                  // 举报分类 ID
    pub content: String,                   // 举报详细描述/内容
    pub status: i16,                       // 状态码: 0: 待处理 1: 处理中 2: 已处理 3: 已拒绝
    pub handler_id: Option<i64>,           // 处理人/审核员 ID
    pub audit_reason: Option<String>,      // 审核原因/违规判定说明
    pub handle_result: Option<String>,     // 处理结果回执 (例如：已下架、已警告、驳回等处理详情)
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 举报记录表字段常量定义
/// * `desc`: `严格与 CocReportRecord 结构体字段顺序和名称对齐`
pub const REPORT_RECORD_COLUMNS: &str = "\
    id, uid, app_id, content_id, classify_id, content, status, \
    handler_id, audit_reason, handle_result, \
    is_deleted, created_at, updated_at, deleted_at\
    ";

//////// END
