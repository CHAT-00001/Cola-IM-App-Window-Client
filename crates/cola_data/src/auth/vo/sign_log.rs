// cola_data/src/code/vo/sign_log.rs  -- 数据中心 - AUTH - vo - 登录类型
// 2026/7/28 10:28

////////

use crate::auth::info::session::SessionInfo;
use crate::cola_user::info::user::UserInfo;
use serde::Serialize;

////////

/// # [VO] - 登录日志
/// * `code`       存储认证令牌信息（access_token, refresh_token, 过期时间）
/// * `user_info`   用户资料
/// * `is_new_user` 是否为新注册用户
#[derive(Debug, Serialize, Clone)]
pub struct SignLogVo {
    pub auth: SessionInfo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}

////////

/// # [Response] - API 响应包装
#[derive(Debug, Serialize, Clone)]
pub struct SignResponse(pub SignLogVo);

// 构造实现
impl Default for SignResponse {
    ////////

    /// # [CASE] - 默认
    fn default() -> Self {
        Self(SignLogVo {
            auth: SessionInfo::default(),
            user_info: UserInfo::default(),
            is_new_user: false,
        })
    }
}

// 构造视图对象
impl SignLogVo {
    pub fn new(auth: SessionInfo, user_info: UserInfo, is_new_user: bool) -> Self {
        Self {
            auth,
            user_info,
            is_new_user,
        }
    }
}

//////// END
