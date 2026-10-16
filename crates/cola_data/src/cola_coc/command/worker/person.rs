//  cola_data/src/video/cmd/worker/person.rs -- 数据 - COC - cmd - 工作人员 - 人员
// 2026/9/1 05:31 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 运营中心 - 人员命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonCommand {
    pub send_id: String,        // 发送 ID
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub video_id: i64,          // 视频 ID
    pub remark: Option<String>, // 备注
    pub sort: Option<i64>,      // 排序
}
