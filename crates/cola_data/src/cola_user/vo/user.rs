// cola_date/src/user/vo/user.rs  --
// 数据 - USER - VO - 用户VO
// 2026/6/18 09:58

////////

use crate::cola_user::info::user::UserInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 用户 视图模型
/// * `desc`: 用户资料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVo {
    #[serde(flatten)]
    pub info: UserInfo, // 用户基础信息(建议也平铺，与之前的DynamicVo保持风格统一)
    pub label: String,      // 用户印象标签
    pub lever: String,      // 用户等级
    pub is_following: bool, // 是否关注
    pub is_online: bool,    // 是否在线
    pub is_streaming: bool, // 是否在直播
}

// 构造函数
impl UserVo {
    /// # [CASE] - 标准构造
    /// 用于在 Service 层完成所有状态查询后，组装成 VO
    pub fn new(info: UserInfo, is_following: bool, is_online: bool, is_streaming: bool) -> Self {
        Self {
            info,
            label: String::from("暂无标签"), // 默认值
            lever: String::from("Lv.1"),     // 默认值
            is_following,
            is_online,
            is_streaming,
        }
    }

    /// # [CASE] - 匿名/未登录/缺省构造
    /// 当用户状态未知或为访客时调用
    pub fn empty() -> Self {
        Self {
            info: UserInfo::default(), // 假设UserInfo有default
            label: String::new(),
            lever: String::new(),
            is_following: false,
            is_online: false,
            is_streaming: false,
        }
    }
}

//////// END
