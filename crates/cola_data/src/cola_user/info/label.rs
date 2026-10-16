// video/src/router/models/label.rs -- 用户标签
// 2026-03-11 10:10:41

use serde::{Deserialize, Serialize};

/// # 用户标签模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLabelModel {
    pub id: i64,         // ID
    pub name: String,    // 名称
    pub name_en: String, // 越英文名称
    pub color: String,   // 颜色
}
