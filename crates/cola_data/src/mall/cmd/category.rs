// cola_data/src/new/cmd/cola_fs.rs  -- 数据中心 - VIDEO - Command - 分类
// 2026/5/22 19:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 短视频 - 分类创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCategoryCommand {
    pub name: String,        // 分类名称
    pub name_en: String,     // 英文名称
    pub description: String, // 描述
}

//////// END
