// code/src/model/vo/code  -- 可乐验证中心 -  Model - Vo - 认证上下文
// 2026/4/16 07:49

////////

use crate::auth::info::session::SessionInfo;
use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [VO] - 无状态Auth结构体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuthContextVo {
    pub uid: Option<i64>,      // 核心：当前登录用户 ID
    pub roles: Vec<String>,    // 扩展：权限角色
    pub device_id: String,     // 安全：设备指纹
    pub is_anonymous: bool,    // 状态：是否为游客
    pub access_token: String,  // 访问令牌
    pub refresh_token: String, // 刷新令牌
}

// 构造函数
impl AuthContextVo {
    /// # 组装器：从 SessionInfo 组装认证上下文
    pub fn from_session(session: SessionInfo, uid: i64, device_id: String) -> Self {
        Self {
            uid: Some(uid),
            roles: Vec::new(), // 可在此处从 DB 查询角色进行填充
            device_id,
            is_anonymous: false,
            access_token: session.access_token,
            refresh_token: session.refresh_token,
        }
    }
}

////////
