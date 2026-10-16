// data/src/cola_coc/info/report/classify.rs -- 数据 - COC - info - 举报 - 分类信息
// 2026/9/1 05:20 Created.

////////

use crate::cola_coc::entity::report::classify::CocReportClassify;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 运营中心 - 举报分类信息
/// * `desc`: `安全的举报分类信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportClassifyInfo {
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

// 构造实现
impl ReportClassifyInfo {
    //

    ////////

    /// 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“举报分类不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            list_order: 0,
            name: "举报分类不存在或已被删除".to_string(),
            name_zh: None,
            status: 0,
            is_deleted: Some(true),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    /// 2. [FROM] - 转换
    /// * `desc`: `纯净的从数据库实体转换为举报分类域模型`
    pub fn from_entity(entity: CocReportClassify) -> Self {
        Self {
            id: entity.id,
            list_order: entity.list_order,
            name: entity.name,
            name_zh: entity.name_zh,
            status: entity.status,
            is_deleted: entity.is_deleted,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
