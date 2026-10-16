// cola_data/src/user/info/profile.rs
// 数据中心 - USER - info - 资料名片
// 2026/8/6 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 资料名片（强社交模型，区别于 UserInfo）
/// * `desc`: 独立的资料名片，包含昵称、头像、背景、签名等社交展示字段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub user_id: i64,            // 用户 ID
    pub nickname: String,        // 昵称
    pub avatar: String,          // 头像
    pub bg_img: String,          // 背景图
    pub signature: String,       // 签名
    pub birthday: Option<i64>,   // 生日
    pub sex: Option<i16>,        // 性别
    pub email: Option<String>,   // 邮箱
    pub phone: Option<String>,   // 电话
    pub sns_url: Option<String>, // 社交链接
    pub label: Option<String>,   // 印象标签
    pub add_time: i64,           // 创建时间
    pub upd_time: i64,           // 更新时间
}

//////// END
