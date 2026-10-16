// video/src/error.rs  -- DATA - APP - 错误码
// 2026/4/13 05:08 Created.

////////

/// # [CODE] - 错误码
/// * `desc`: `保持你原来的模块命名，我们可以直接在这里扩展`
pub mod error {
    pub const SUCCESS: i32 = 0;  // Ok
    pub const PARAM_ERROR: i32 = 4000;  // 参数错误

    // --- 认证与授权 (Auth) ---
    pub const NOT_LOGIN: i32 = 4001; // 未登录
    pub const UNAUTHORIZED: i32 = 4002; // 未知权限
    pub const NO_PERMISSION: i32 = 4030; // 无权限
    pub const FORBIDDEN: i32 = 4030; // 别名

    // --- 用户状态 (User) ---
    pub const USER_NOT_FOUND: i32 = 4002; // 用户不存在
    pub const USER_DELETED: i32 = 4003; // 用户已删除
    pub const USER_BANNED: i32 = 4004; // 用户已被封禁

    // --- 基础错误 ---
    pub const NOT_FOUND: i32 = 4004; // 不存在
    pub const INVALID_PARAM: i32 = 4100;
    pub const INTERNAL_ERROR: i32 = 5000;
}

pub use error::*;

//////// END
