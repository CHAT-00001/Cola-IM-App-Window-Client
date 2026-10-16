// cola_data/src/coc/cmd/ticket/record.rs -- 数据 - COC - Command - 工单 - 记录命令
// 2026/9/1 05:25 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 运营中心 - 工单命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketCommand {
    pub send_id: String,          // 发送 ID
    pub user_id: i64,             // 用户 ID
    pub video_id: i64,            // 视频 ID
    pub album_id: Option<i64>,    // 专辑 ID
    pub target: String,           // 分享目标
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}
