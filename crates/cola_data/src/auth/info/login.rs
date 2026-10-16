// cola_data/src/code/info/port.rs  -- 可乐数据中心 - AUTH - Info - 会话信息
// 2026/6/9 10:07

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 登录响应信息
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginInfo {
    pub access_token: String,              // 临时 Token (JWT)
    pub refresh_token: String,             // 刷新 Token
    pub access_expired_at: DateTime<Utc>,  // 临时 Token 过期时间
    pub refresh_expired_at: DateTime<Utc>, // 长期 Token 过期时间
}

/// # [INFO] - 刷新ACCESS_TOKEN
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessTokenInfo {
    pub access_token: String,             // 临时 Token (JWT)
    pub access_expired_at: DateTime<Utc>, // 临时 Token 过期时间
}

//////// EDN
