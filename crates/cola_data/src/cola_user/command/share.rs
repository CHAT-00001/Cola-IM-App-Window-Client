// cola_data/src/user/cmd/ticket
// 数据中心 - USER - cmd - 分享
// 2026/8/6 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户主页分享命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShareUserCommand {
    pub user_id: i64,             // 被分享的用户ID
    pub share_type: i16,          // 分享类型: 1=站内 2=站外
    pub platform: Option<String>, // 分享平台: wechat/weibo/qq
    pub remark: String,           // 备注
}

//////// END
