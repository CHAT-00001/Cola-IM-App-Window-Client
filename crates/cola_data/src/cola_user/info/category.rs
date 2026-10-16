// /category.rs
//
// 2026/8/4 01:31 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 - 分类 - 信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCategoryInfo {
    pub id: i64,      // 分类ID
    pub name: String, // 分类名称
    pub status: i16,  // 状态: 1有效 0失效
}
