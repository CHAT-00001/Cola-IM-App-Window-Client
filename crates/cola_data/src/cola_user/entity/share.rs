// cola_data/src/user/entity/share.rs -- 数据 - USER - entity - 分享记录表
// 2026/8/5 20:36 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户中心 - 分享记录表
/// * `pg schema`: `cola.user` -- PG 模式
/// * `table name`: `share_record` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserShareEntity {
    pub id: i64,                            // ID (自增 / 雪花)
    pub _id: Option<String>,                // UUID v4
    pub uid: i64,                           // 举报人用户ID (操作者)
    pub target_type: i16,                   // 举报目标类型: 1.用户 2.视频 3.评论 4.动态...
    pub target_id: i64,                     // 举报目标ID (对应用户ID/视频ID等)
    pub name: Option<String>,               // 举报时的目标快照名称（如当时的用户昵称/视频标题）
    pub content: Option<String>,            // 举报详细内容/理由描述
    pub images: Option<Vec<String>>,        // 举报凭证图片凭证
    pub review_uid: Option<i64>,            // 审核员UID
    pub review_reason: Option<String>,      // 审核意见/驳回原因
    pub review_status: i16,                 // 审核状态码: 0.待审核 1.审核中 2.已通过 3.已驳回
    pub is_starred: bool,                   // 是否标星/加急: 默认false
    pub is_deleted: bool,                   // 是否删除: 默认false
    pub status: i16,                        // 状态码: 0.无效 1.有效
    pub add_time: i64,                      // 添加时间（PHP旧版兼容，改为i64防止2038问题）
    pub upd_time: i64,                      // 更新时间（PHP旧版兼容）
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,  // 删除时间（软删除）
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
}

////////

/// # [COLUMNS] - sqlx数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_SHARE_COLUMNS: &str = r#"
    id, _id, uid, target_type, target_id, name, content, images,
    review_uid, review_reason, review_status,
    is_starred, is_deleted, status,
    add_time, upd_time, created_at, updated_at, deleted_at, reviewed_at
"#;

//////// END
