// cola_data/src/video/entity/view/history.rs
// 数据 - VIDEO - entity - view - 浏览历史表
// 2026/8/2 14:22 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 浏览历史记录表
/// * `pg schema`: `video`
/// * `table name`: `view_history`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoViewHistoryEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 用户ID
    pub video_id: i64,                     // 视频ID
    pub time: i32,                         // 观看到的时间
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态码:  0失效  1有效
    pub is_deleted: bool,                  // 是否删除 默认false
    pub addtime: i64,                      // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_VIEW_COLUMNS: &str = r#"
    id, user_id, video_id, time, remark, addtime, status, created_at, updated_at
"#;

//////// END
