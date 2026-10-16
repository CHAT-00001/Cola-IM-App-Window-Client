// cola_data/src/new/cola_dynamic/hotlist.rs  -- VIDEO - Command - 推荐命令
// 2026/5/22 20:45 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 短视频 - 推荐命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendCommand {
    pub send_id: String,        // 发送 ID
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub video_id: i64,          // 视频 ID
    pub remark: Option<String>, // 备注
}
