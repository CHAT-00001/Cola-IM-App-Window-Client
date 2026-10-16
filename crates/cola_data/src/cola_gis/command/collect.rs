// cola_data/src/cola_gis/cmd/count  -- 可乐数据中心 - GIS - Command - 收藏
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - POI 收藏 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiCollectCommand {
    pub send_id: String,        // 同步 ID
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub poi_id: i64,            // 兴趣点 ID
    pub remark: Option<String>, // 备注
    pub sort: Option<i64>,      // 排序
}

//////// END
