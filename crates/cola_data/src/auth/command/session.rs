// cola_data/src/code/cmd/service.rs  -- 数据 - AUTH - Command - service - mod
// 2026/06/05 06:50

////////

pub mod add;
pub mod refresh;
pub mod signout;

use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [COMMAND] - 会话命令
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SessionCommand {
    pub client_id: String,                                 // 客户端 ID
    pub device_id: String,                                 // 设备 ID
    pub access_token: String,                              // jwt token
    pub refresh_token: String,                             // aes token
    pub access_expires_at: chrono::DateTime<chrono::Utc>,  // 临时token过期时间
    pub refresh_expires_at: chrono::DateTime<chrono::Utc>, // 刷新Token过期时间
    pub last_active_at: chrono::DateTime<chrono::Utc>,     // 最后一次激活时间(更新时间)
}

impl SessionCommand {
    /// # [CONFIG] - access_token 统一过期配置（天）
    /// * `desc`: 开发初期设为 10 天，避免频繁登录
    pub const ACCESS_TOKEN_TTL_DAYS: i64 = 10;

    /// # [CONFIG] - refresh_token 统一过期配置（天）
    pub const REFRESH_TOKEN_TTL_DAYS: i64 = 180;
}

// 构造函数
impl SessionCommand {
    //

    ////////

    /// # [BUILD] - 构造函数：基于基本信息生成标准有效期 Token 命令
    /// * `access_expires_at`: 由 `Self::ACCESS_TOKEN_TTL_DAYS` 统一控制
    pub fn new_with_defaults(access_token: String, refresh_token: String) -> Self {
        let now = chrono::Utc::now();
        let client_id = "a".to_string();
        let device_id = "b".to_string();
        let access_expires_at = now + chrono::Duration::days(Self::ACCESS_TOKEN_TTL_DAYS);
        let refresh_expires_at = now + chrono::Duration::days(Self::REFRESH_TOKEN_TTL_DAYS);
        let last_active_at = now;

        Self {
            client_id,
            device_id,
            access_token,
            refresh_token,
            access_expires_at,
            refresh_expires_at,
            last_active_at,
        }
    }
}

//////// END
