// cola_data/src/cola_gis/entity/report.rs  -- GIS - entity - 兴趣点举报/类型
// 2026/4/24 18:07 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 兴趣点 举报 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_report`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct PoiReport {
    pub id: i64,             // 检举 ID
    pub uid: i64,            // 举报用户 ID
    pub to_uid: i64,         // 被举报用户 ID
    pub poi_id: i64,         // 被举报兴趣点 ID
    pub content: String,     // 举报内容
    pub status: i16,         // 举报状态 0: 待处理 1: 巄理中 2: 已处理 3: 已拒绝
    pub create_time: String, // 创建时间
    pub update_time: String, // 更新时间
}

/// # [ENTITY] - 举报分类
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_report_classify`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoReportClassify {
    pub id: i16,             // 举报类型 ID
    pub list_order: i32,     // 列表排序
    pub name: String,        // 举报类型名称
    pub create_time: String, // 创建时间
    pub update_time: String, // 更新时间
}

//////// END
