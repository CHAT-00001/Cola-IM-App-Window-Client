// cola_data/src/user/vo/role.rs
// 可乐数据中心 - 用户 - vo - 角色视图对象
// 2026/8/6 Created.

////////

use crate::cola_user::info::role::RoleInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 角色视图对象
/// * `desc`: 角色展示给前端的视图模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleVo {
    #[serde(flatten)]
    pub info: RoleInfo, // 角色基础信息
    pub label: String,    // 角色标签
    pub users_count: i64, // 拥有该角色的用户数量
}

impl RoleVo {
    ////////

    /// # 1. [CASE] - 标准构造
    pub fn new(info: RoleInfo) -> Self {
        Self {
            info,
            label: String::from("普通角色"),
            users_count: 0,
        }
    }
}

//////// END
