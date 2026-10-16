// cola_data/src/new/entity/banned/download.rs  --
// 数据 - VIDEO - entity - banned - 下载
// 2026/8/1 15:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频 用户 封禁 下载
/// * `pg schema`: `video`
/// * `table name`: `banned_danmaku`
#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct VideoBannedDownloadEntity {
    pub uid: i64,                          // 用户 ID
    pub operator_uid: Option<String>,      // 操作者 ID
    pub begin_at: Option<DateTime<Utc>>,   // 开始时间
    pub end_at: Option<DateTime<Utc>>,     // 结束时间
    pub reason: Option<String>,            // 原因
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 是否删除 / 同步状态
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 软删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_BANNED_DOWNLOAD_COLUMNS: &str = r#"
    uid, operator_uid, begin_at, end_at, reason, status,
    is_deleted, created_at, updated_at, deleted_at
"#;

// END
