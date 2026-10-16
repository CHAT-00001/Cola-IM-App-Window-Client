// cola_data/src/cola_gis/cmd/add  -- 可乐数据中心 - GIS - Command - 分享
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - POI 分享 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShareCommand {
    pub send_id: String,          // 同步 ID
    pub user_id: i64,             // 分享者 ID
    pub poi_id: i64,              // POI ID
    pub album_id: Option<i64>,    // 专辑 ID
    pub target: String,           // 目标
    pub report_tags: Vec<String>, // 标签
    pub remark: Option<String>,   // 备注
}

//////// END
