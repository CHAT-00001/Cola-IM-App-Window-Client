// /config.rs  -- 用户 配置 info
// 2026/6/11 02:17

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfigInfo {
    pub is_shop: i16, // 是否开店
}

impl UserConfigInfo {
    /// 构造函数
    pub fn new() -> Self {
        Self { is_shop: 0 }
    }
}

//////// END
