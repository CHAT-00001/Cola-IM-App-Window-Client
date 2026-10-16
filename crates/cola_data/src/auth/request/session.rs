// cola_data/src/code/request/service.rs  -- 数据 - AUTH - request - 会话
// 2026/7/30 16:10

////////

use serde::{Deserialize, Serialize};

////////

/// # [AUTH] - 客户端认证请求
/// * `desc`: `网关接收客户端提交的 code 信息`
/// `不可信数据，需要经过验证后生成 SessionContext`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthSessionRequest {
    // 客户端携带的访问令牌
    pub access_token: Option<String>,

    // 刷新令牌
    pub refresh_token: Option<String>,

    // 设备信息
    pub device_id: Option<String>,
}

impl AuthSessionRequest {
    /// 是否存在登录信息
    pub fn has_token(&self) -> bool {
        self.access_token
            .as_deref()
            .is_some_and(|v| !v.trim().is_empty())
    }
}

////////

/// # [AUTH] - 服务端可信会话上下文
///
/// token验证成功后生成
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionContext {
    /// 操作者ID
    pub uid: i64,

    /// 权限角色
    pub iam_roles: Vec<String>,

    /// 设备指纹
    pub device_id: String,

    /// 是否游客
    pub is_anonymous: bool,

    /// 原始token
    #[serde(skip_serializing)]
    pub access_token: String,
}

impl SessionContext {
    /// 游客
    pub fn anonymous() -> Self {
        Self {
            uid: 0,
            iam_roles: Vec::new(),
            device_id: String::new(),
            is_anonymous: true,
            access_token: String::new(),
        }
    }

    /// token解析后构造
    pub fn from_token(
        uid: i64,
        roles: Vec<String>,
        device_id: String,
        access_token: String,
    ) -> Self {
        Self {
            uid,
            iam_roles: roles,
            device_id,
            is_anonymous: false,
            access_token,
        }
    }

    /// 是否已登录
    pub fn is_login(&self) -> bool {
        !self.is_anonymous
    }

    /// 是否拥有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.iam_roles.iter().any(|r| r == role)
    }
}

//////// END
