// cola_data/src/code/info/code  -- 数据 - AUTH - Info - 验证壳信息 构造器
// 2026/4/16 07:49

////////

use crate::auth::command::phone::PhoneLoginCommand;
use crate::cola_user::info::permission::UserPermissionContext;
use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [AUTH] - 无状态Auth结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct AuthContext {
    pub uid: i64,               // 核心：当前登录用户 ID
    pub iam_roles: Vec<String>, // 扩展：权限角色
    pub device_id: String,      // 安全：设备指纹
    pub is_anonymous: bool,     // 状态：是否为游客
    pub access_token: String,   // 访问令牌
    pub refresh_token: String,  // 刷新令牌

    // 🆕 权限信息（由网关层聚合）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_context: Option<UserPermissionContext>, // 权限上下文
}

impl AuthContext {
    // 💡

    ////////

    /// #[BUILDER] 构造函数 - 生成 AuthContext 实例
    pub fn new(uid: i64, access_token: String, refresh_token: String, device_id: String) -> Self {
        // 自动判断是否匿名：没有 uid 就是匿名游客
        let is_anonymous = false;

        Self {
            uid,
            iam_roles: Vec::new(), // 默认空角色，后续可扩展
            device_id,
            is_anonymous,
            access_token,
            refresh_token,
            permission_context: None,
        }
    }

    /// # [CHECKER] - 检查权限等级
    pub fn has_permission_level(&self, required_level: i16) -> bool {
        self.permission_context
            .as_ref()
            .map(|ctx| ctx.has_level(required_level))
            .unwrap_or(false)
    }

    /// # [CHECKER] - 检查系统权限
    pub fn has_sys_permission(&self, perm: &str) -> bool {
        self.permission_context
            .as_ref()
            .map(|ctx| ctx.has_sys_permission(perm))
            .unwrap_or(false)
    }

    /// # [CHECKER] - 检查角色
    pub fn has_role(&self, role_code: &str) -> bool {
        self.permission_context
            .as_ref()
            .map(|ctx| ctx.has_role(role_code))
            .unwrap_or(false)
    }
}

//////// END
