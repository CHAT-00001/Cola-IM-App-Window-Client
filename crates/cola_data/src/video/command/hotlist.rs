// cola_data/src/video/cmd/hotlist.rs -- 数据 - VIDEO - cmd - 上热门 - mod
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 短视频 - 上热门命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoHotlistCommand {
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub video_id: i64,          // 视频 ID
    pub remark: Option<String>, // 备注
    pub qty: i32,               // 曝光量
    pub time_zone: String,      // 时间区间
    pub older: Option<String>,  // 订单 ID
}
