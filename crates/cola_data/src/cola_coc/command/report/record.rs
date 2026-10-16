// cola_data/src/coc/cmd/report/record.rs -- 数据 - COC - Command - 举报 - 记录命令
// 2026/9/1 05:28 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 运营中心 - 举报命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportCommand {
    pub user_id: i64,             // 用户 ID
    pub app_id: i16,              // 应用 ID
    pub content_id: i64,          // 内容 ID
    pub report_type: String,      // 举报类型
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}

//////// END
