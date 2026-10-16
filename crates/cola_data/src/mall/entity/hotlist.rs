// cola_data/src/video/entity/hotlist.rs -- 数据 - VIDEO - entity - 上热门记录表
// 2026/9/11 17:28 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 - 上热门记录表
/// * `pg schema`: `video`
/// * `table name`: `hotlist`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoHotlistEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub remark: Option<String>,            // 备注 (可选)
    pub status: i16,                       // 状态码: 0失效 1有效
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub add_time: i64,                     // 添加时间(兼容旧版)
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [CONSTANT] - 上热门记录表字段常量定义
pub const VIDEO_HOTLIST_COLUMNS: &str = "\
    id, user_id, video_id, remark, \
    status, is_deleted, \
    add_time, created_at, updated_at, deleted_at\
    ";

//////// END
