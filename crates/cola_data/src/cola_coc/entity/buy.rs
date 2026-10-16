// cola_data/src/video/entity/buy.rs -- 数据 - VIDEO - entity - buy - 购买记录表
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
    pub id: i64,                           // ID (自增 / 雪花)
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub folder_id: i64,                    // 购买夹 ID：默认 0 (主购买夹/未分类)
    pub channel: i16,                      // 通道
    pub remark: Option<String>,            // 备注(可选)
    pub status: i16,                       // 状态码：0失效 1有效
    pub add_time: i32,                     // 添加时间 (兼容旧版)
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 常量字段
pub const VIDEO_BUY_COLUMNS: &str = r#"
    id, user_id, video_id, folder_id, channel_id,
    remark, status, add_time, is_deleted,
    created_at, updated_at, deleted_at,
"#;
//////// END
