// data/src/code/vo/service.rs
// 数据 - AUTH - vo - 会话视图对象
// 2026-07-28 10:26

////////

use crate::auth::info::session::SessionInfo;
use crate::cola_user::info::user::UserInfo;
use serde::Serialize;

////////

/// # [VO] - 登录/注册 视图对象
/// * `desc`: `存储认证令牌信息（access_token, refresh_token, 过期时间）`
#[derive(Debug, Serialize, Clone)]
pub struct SignVo {
    pub auth: SessionInfo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}

////////

/// # [Response] - API 响应包装
#[derive(Debug, Serialize, Clone)]
pub struct SignResponse(pub SignVo);

impl Default for SignResponse {
    fn default() -> Self {
        Self(SignVo {
            auth: SessionInfo::default(),
            user_info: UserInfo::default(),
            is_new_user: false,
        })
    }
}

// 构造实现
impl SignVo {
    //

    ////////

    /// # [BUILD] - 新的
    pub fn new(auth: SessionInfo, user_info: UserInfo, is_new_user: bool) -> Self {
        Self {
            auth,
            user_info,
            is_new_user,
        }
    }
}

//////// END
