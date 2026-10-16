// cola_data/src/user/cmd/profile.rs -- 数据 - USER - cmd - 资料命令
// 2026/8/6 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户 资料名片 创建/更新命令
/// * `desc`: 强社交资料模型（与 UserInfo 不同，这是独立的资料名片）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileCommand {
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
}

//////// END
