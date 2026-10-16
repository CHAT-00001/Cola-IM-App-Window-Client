// cola_data/src/user/info/vip.rs -- 数据中心 - USER - info - 贵宾信息
// 2026/9/5 07:11 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 - 贵宾信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserVipInfo {
    pub id: i64,                  // 分享记录ID
    pub uid: i64,                 // 分享者ID
    pub target_user_id: i64,      // 被分享的用户ID
    pub share_type: i16,          // 分享类型: 1=站内 2=站外
    pub platform: Option<String>, // 分享平台
    pub remark: String,           // 备注
    pub status: i16,              // 状态码
    pub add_time: i64,            // 添加时间
}

////////

/// # [INFO] - 用户 - 贵宾规则信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserVipRuleInfo {
    pub id: i64,                  // 分享记录ID
    pub uid: i64,                 // 分享者ID
    pub target_user_id: i64,      // 被分享的用户ID
    pub share_type: i16,          // 分享类型: 1=站内 2=站外
    pub platform: Option<String>, // 分享平台
    pub remark: String,           // 备注
    pub status: i16,              // 状态码
    pub add_time: i64,            // 添加时间
}

//////// END
