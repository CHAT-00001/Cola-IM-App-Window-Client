// cola_data/src/video/entity/view.rs -- 数据 - VIDEO - entity - 浏览 - 记录表
// 2026/3/28 05:56 Created.

////////

mod history;
////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 - 浏览记录表
/// * `pg schema`: `video` -- PG模式
/// * `table name`: `view` -- 数据表
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoViewEntity {
    pub id: i64,                           // ID
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 用户ID
    pub video_id: i64,                     // 视频ID
    pub time: i32,                         // 观看到的时间
    pub remark: Option<String>,            // 备注
    pub is_done: Option<bool>,             // 是否观看完毕
    pub count: Option<i16>,                // 观看次数
    pub addtime: i64,                      // 创建时间（兼容PHP旧版）
    pub updtime: Option<i64>,              // 更新时间（兼容PHP旧版）
    pub status: i16,                       // 状态码:  0失效 1有效
    pub is_deleted: Option<bool>,          // 是否删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_VIEW_COLUMNS: &str = r#"
    id, _id, user_id, video_id,
    time, remark, is_done, count,
    addtime, updtime, status, is_deleted,
    created_at, updated_at, deleted_at
"#;

//////// END
