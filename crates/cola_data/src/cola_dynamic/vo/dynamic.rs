// cola_data/src/cola_dynamic/vo/cola_dynamic.rs  -- 动态 - VO - 动态
// 2026/6/19 16:50

////////

use crate::cola_dynamic::info::dynamic::{DynamicInfo, Media};
use crate::cola_user::vo::user::UserVo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 动态 视图模型
/// * `desc` 需要兼容旧版PHP字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicVo {
    #[serde(flatten)]
    pub info: DynamicInfo, // 动态元信息(平铺)
    pub user: UserVo,     // 用户信息
    pub distance: String, // 球面距离
    pub is_like: bool,    // 是否点赞
    pub is_collect: bool, // 是否收藏
}

// 构造函数
impl DynamicVo {
    ////////

    /// # [CASE] - 新
    pub fn new(
        info: DynamicInfo,
        user: UserVo,
        distance: String,
        is_like: bool,
        is_collect: bool,
    ) -> Self {
        Self {
            info,
            user,
            distance: "0m".to_string(),
            is_like,
            is_collect,
        }
    }

    ////////

    /// # [CASE] - 动态不存在
    pub fn no_found(user: UserVo) -> Self {
        Self {
            info: DynamicInfo::empty(), // 调用构造函数
            user,
            distance: "0m".to_string(),
            is_like: false,
            is_collect: false,
        }
    }
}

//////// END
