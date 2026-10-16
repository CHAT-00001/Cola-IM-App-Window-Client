// cola_data/src/coc/cmd/report/category.rs  -- 数据中心 - COC - Command - 举报 - 分类
// 2026/5/22 19:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 运营中心 - 举报分类创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCommand {
    pub name: String,        // 分类名称
    pub name_en: String,     // 英文名称
    pub description: String, // 描述
}

//////// END
