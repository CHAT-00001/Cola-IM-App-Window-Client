// cola_data/src/cola_gis/cmd/report.rs  -- 可乐数据中心 - GIS - Command - 举报
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - POI 举报 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiReportCommand {
    pub send_id: String,          // 同步 ID
    pub user_id: i64,             // 用户 ID
    pub content: Option<String>,  // 内容
    pub poi_id: i64,              // POI ID
    pub report_type: String,      // 举报类型
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}

//////// END
