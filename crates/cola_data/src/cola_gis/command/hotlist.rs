// cola_data/src/cola_gis/cmd/hotlist.rs -- 数据 - 可乐GIS - cmd - 上热门 - mod
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - POI 上热门 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiHotlistCommand {
    pub send_id: String,        // 鍙戦€?ID
    pub user_id: i64,           // 鐢ㄦ埛 ID
    pub album_id: Option<i64>,  // 涓撹緫 ID
    pub video_id: i64,          // 瑙嗛 ID
    pub remark: Option<String>, // 澶囨敞
    pub qty: i32,               // 鏇濆厜閲?
    pub time_zone: String,      // 鏃堕棿鍖洪棿
    pub older: Option<String>,  // 璁㈠崟 ID
}

//////// END
