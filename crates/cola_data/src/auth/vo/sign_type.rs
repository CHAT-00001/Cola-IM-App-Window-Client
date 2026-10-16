// cola_data/src/code/vo/sign_type.rs  -- 数据中心 - AUTH - vo - 登录类型
// 2026/7/28 10:25

////////

use crate::auth::info::sign_type::SignTypeInfo;
use crate::cola_user::info::user::UserInfo;
use serde::Serialize;

////////

/// # [VO] - 登录类型 视图对象
#[derive(Debug, Serialize, Clone)]
pub struct SignTypeVo {
    pub info: SignTypeInfo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}

////////

/// # [Response] - API 响应包装
#[derive(Debug, Serialize, Clone)]
pub struct SignResponse(pub SignTypeVo);

impl Default for SignResponse {
    fn default() -> Self {
        Self(SignTypeVo {
            info: SignTypeInfo::default(),
            user_info: UserInfo::default(),
            is_new_user: false,
        })
    }
}

impl SignTypeVo {
    pub fn new(info: SignTypeInfo, user_info: UserInfo, is_new_user: bool) -> Self {
        Self {
            info,
            user_info,
            is_new_user,
        }
    }
}

//////// END
