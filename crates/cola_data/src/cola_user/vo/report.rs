// cola_data/src/user/vo/report.rs
// 可乐数据中心 - 用户 - vo - 审核端举报内容视图对象
// 2026/8/5 20:17 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 审核端举报内容视图对象
/// * `desc`: `面向审核员，包含关联的用户资料和目标详情`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserReportContentVo {
    // -- 举报基础信息
    pub id: i64,                     // ID
    pub _id: Option<String>,         // UUID v4
    pub target_type: i16,            // 举报目标类型: 1.用户 2.视频 3.评论等
    pub target_id: i64,              // 举报目标ID
    pub name: Option<String>,        // 目标快照名称
    pub content: Option<String>,     // 举报内容详情
    pub images: Option<Vec<String>>, // 举报凭证图片

    // -- 举报人（UID）相关扩展资料（拼接展示）
    pub reporter_uid: i64,         // 举报人UID
    pub reporter_nickname: String, // 举报人昵称
    pub reporter_avatar: String,   // 举报人头像

    // -- 审核相关信息
    pub review_uid: Option<i64>,            // 审核员UID
    pub review_reason: Option<String>,      // 审核意见/驳回原因
    pub review_status: i16,                 // 审核状态码: 0.待审核 1.审核中 2.已通过 3.已驳回
    pub is_starred: bool,                   // 是否标星/加急
    pub status: i16,                        // 状态码
    pub add_time: i64,                      // 添加时间
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

//////// END
