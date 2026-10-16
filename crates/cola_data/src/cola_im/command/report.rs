// cola_data/src/im/cmd/report.rs  -- 数据 - 可乐IM - Command - 举报 - mod
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - IM用户资料举报命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImProfileReportCommand {
    pub send_id: String,          // 发送 ID
    pub user_id: i64,             // 用户 ID
    pub album_id: Option<i64>,    // 专辑 ID
    pub profile_id: i64,          // 资料 ID
    pub report_type: String,      // 举报类型
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}

//////// END
