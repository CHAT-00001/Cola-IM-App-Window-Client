// data/src/cola_coc/info/report/record.rs -- 数据 - COC - info - 举报 - 记录信息
// 2026/9/1 05:20 Created.

////////

use crate::cola_coc::entity::report::record::CocReportEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 运营中心 - 举报记录信息
/// * `desc`: `安全的举报记录信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportRecordInfo {
    pub id: i64,                           // 检举 ID
    pub uid: i64,                          // 用户 ID (举报人)
    pub app_id: i16,                       // 应用 ID
    pub content_id: i64,                   // 被举报的内容 ID
    pub classify_id: i16,                  // 举报分类 ID
    pub content: String,                   // 举报详细描述/内容
    pub status: i16,                       // 状态码: 0: 待处理 1: 处理中 2: 已处理 3: 已拒绝
    pub handler_id: Option<i64>,           // 处理人/审核员 ID
    pub audit_reason: Option<String>,      // 审核原因/违规判定说明
    pub handle_result: Option<String>,     // 处理结果回执
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

// 构造实现
impl ReportRecordInfo {
    //

    ////////

    /// 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“举报记录不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            uid: 0,
            app_id: 0,
            content_id: 0,
            classify_id: 0,
            content: "举报记录不存在或已被删除".to_string(),
            status: 0,
            handler_id: None,
            audit_reason: None,
            handle_result: None,
            is_deleted: Some(true),
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    /// 2. [FROM] - 转换
    /// * `desc`: `纯净的从数据库实体转换为举报记录域模型`
    pub fn from_entity(entity: CocReportEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            app_id: entity.app_id,
            content_id: entity.content_id,
            classify_id: entity.classify_id,
            content: entity.content,
            status: entity.status,
            handler_id: entity.handler_id,
            audit_reason: entity.audit_reason,
            handle_result: entity.handle_result,
            is_deleted: entity.is_deleted,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
