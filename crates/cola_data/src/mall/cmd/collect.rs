// cola_data/src/video/cmd/collect.rs -- 数据 - VIDEO - cmd - 收藏 - mod
// 2026/5/22 20:45 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 短视频 - 收藏命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectCommand {
    pub send_id: String,        // 发送 ID
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub video_id: i64,          // 视频 ID
    pub remark: Option<String>, // 备注
    pub sort: Option<i64>,      // 排序
}
