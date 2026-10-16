// cola_data/src/code/info/service.rs  --  可乐数据中心 - AUTH - Info - 会话信息
// 2026/4/12 03:20 by wx: cestbon10080

////////

use crate::auth::command::session::SessionCommand;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 会话信息
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub access_token: String,              // 临时 Token (JWT)
    pub refresh_token: String,             // 刷新 Token
    pub access_expires_at: DateTime<Utc>,  // 临时 Token 过期时间
    pub refresh_expires_at: DateTime<Utc>, // 长期 Token 过期时间
}

////////

/// # [INFO] - 刷新ACCESS_TOKEN
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessTokenInfo {
    pub access_token: String,             // 临时 Token (JWT)
    pub access_expired_at: DateTime<Utc>, // 临时 Token 过期时间
}

impl From<SessionCommand> for SessionInfo {
    fn from(cmd: SessionCommand) -> Self {
        Self {
            access_token: cmd.access_token,
            refresh_token: cmd.refresh_token,
            access_expires_at: cmd.access_expires_at,
            refresh_expires_at: cmd.refresh_expires_at,
        }
    }
}

//////// EDN
