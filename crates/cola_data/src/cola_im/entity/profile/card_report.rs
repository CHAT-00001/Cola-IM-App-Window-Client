// video/src/cola_live/gate_grpc/report.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 举报视频类型
// 2026/4/24 18:07 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// # ENTITY - 视频举报
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoReport {
    pub id: i64,             // 检举 ID
    pub uid: i64,            // 举报用户 ID
    pub to_uid: i64,         // 被举报用户 ID
    pub video_id: i64,       // 被举报视频 ID
    pub content: String,     // 举报内容
    pub status: i16,         // 举报状态 0: 待处理 1: 巄理中 2: 已处理 3: 已拒绝
    pub create_time: String, // 创建时间
    pub update_time: String, // 更新时间
}

/// # ENTITY - 视频举报分类
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoReportClassify {
    pub id: i16,             // 举报类型 ID
    pub list_order: i32,     // 列表排序
    pub name: String,        // 举报类型名称
    pub create_time: String, // 创建时间
    pub update_time: String, // 更新时间
}
