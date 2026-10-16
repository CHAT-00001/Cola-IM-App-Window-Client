// cola_data/src/cola_user/info/permission.rs -- 数据 - USER - Info - 权限信息
// 2026/9/19 00:59 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户权限上下文
/// * `desc`: `包含用户权限等级、角色、权限列表`
/// * `scope`: 由网关层聚合，下层直接使用
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPermissionContext {
    pub uid: i64,                     // 用户ID
    pub base_level: i16,              // 权限等级 1-12
    pub level_name: String,           // 'GUEST', 'USER', 'VIP', ...
    pub roles: Vec<String>,           // ['broadcaster', 'content_reviewer']
    pub sys_permissions: Vec<String>, // 聚合的系统权限
    pub is_vip: bool,                 // base_level >= 3
    pub is_creator: bool,             // base_level >= 4
    pub is_operator: bool,            // base_level >= 6
    pub is_admin: bool,               // base_level >= 10
}

////////

impl UserPermissionContext {
    /// # 1. [BUILDER] - 游客权限
    /// * `desc`: `未登录用户的权限上下文`
    pub fn guest() -> Self {
        Self {
            uid: 0,
            base_level: 1,
            level_name: "GUEST".to_string(),
            roles: vec![],
            sys_permissions: vec![],
            is_vip: false,
            is_creator: false,
            is_operator: false,
            is_admin: false,
        }
    }

    /// # 2. [CHECKER] - 检查权限等级
    /// * `desc`: `检查用户是否拥有指定权限等级`
    pub fn has_level(&self, required_level: i16) -> bool {
        self.base_level >= required_level
    }

    /// # 3. [CHECKER] - 检查系统权限
    /// * `desc`: `检查用户是否拥有指定系统权限`
    pub fn has_sys_permission(&self, perm: &str) -> bool {
        self.sys_permissions.contains(&perm.to_string())
    }

    /// # 4. [CHECKER] - 检查角色
    /// * `desc`: `检查用户是否拥有指定角色`
    pub fn has_role(&self, role_code: &str) -> bool {
        self.roles.contains(&role_code.to_string())
    }

    ////////
}

//////// END
