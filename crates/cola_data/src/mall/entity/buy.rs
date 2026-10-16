// cola_data/src/video/entity/buy.rs -- 数据 - VIDEO - entity - 购买记录表
// 2026/8/3 00:17 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 购买记录表
/// * `pg schema`: `video`
/// * `table name`: `buy`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoBuyEntity {
    pub id: i64,                           // 购买记录自增 ID
    pub uid: i64,                          // 购买发起者用户 ID
    pub video_id: i64,                     // 被购买的短视频 ID
    pub folder_id: i64,                    // 购买夹 ID：默认 0 (主购买夹/未分类)
    pub channel: i16,                      // 通道
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态：0失效 1有效
    pub add_time: i32,                     // 添加时间 (兼容旧版 PHP)
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 常量字段
pub const VIDEO_BUY_COLUMNS: &str = r#"
    id, uid, video_id, folder_id, channel_id,
    remark, status, addtime, is_deleted,
    created_at, updated_at, deleted_at,
"#;
//////// END
