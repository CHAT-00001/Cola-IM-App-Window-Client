// cola_data/src/cola_coc/entity/ticke/record.rs -- 数据 - COC - entity - 工单 - 记录表
// 2026/9/1 05:34 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 运营中心 - 工单表
/// * `pg schema`: `cola_coc` -- PG模式
/// * `table name`: `ticket` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CocTicketEntity {
    pub id: i64,                           // 工单 ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4 (对外安全标识)
    pub user_id: i64,                      // 提单用户 ID
    pub title: String,                     // 工单标题
    pub content: String,                   // 工单内容/描述
    pub ticket_type: i16,                  // 工单类型: (1-内容审核, 2-版权申诉, 3-违规举报, 4-其他)
    pub status: i16, // 状态: (0-待处理, 1-处理中, 2-已解决, 3-已驳回, 4-已关闭)
    pub urgency_level: i16, // 加急等级: (0-普通, 1-加急, 2-特急)
    pub required_auth_level: i16, // 所需审核权限等级: (1-初级审核员, 2-高级审核员, 3-运营主管, 4-平台管理员)
    pub current_handler_id: Option<i64>, // 当前处理人/审核员 ID
    pub extra_data: Option<String>, // 扩展业务数据 (JSON格式，用于存放关联的视频ID、动态ID或特定参数)
    pub is_deleted: Option<bool>,   // 逻辑删除
    pub created_at: DateTime<Utc>,  // 创建时间
    pub updated_at: DateTime<Utc>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
/// * `描述`: 给SQLX使用，严格与 TicketEntity 结构体字段对齐
pub const COC_TICKET_COLUMNS: &str = r#"
    id, _id, user_id, title, content, ticket_type, status,
    urgency_level, required_auth_level, current_handler_id,
    extra_data, is_deleted, created_at, updated_at, deleted_at
"#;

//////// END
