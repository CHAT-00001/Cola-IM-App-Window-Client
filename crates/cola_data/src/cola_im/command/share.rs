// cola_data/src/new/cola_dynamic/add  -- VIDEO - Command - 分享命令
// 2026/5/22 20:45 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 短视频 - 分享命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShareCommand {
    pub send_id: String,          // 发送 ID
    pub user_id: i64,             // 用户 ID
    pub video_id: i64,            // 视频 ID
    pub album_id: Option<i64>,    // 专辑 ID
    pub target: String,           // 分享目标
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}
