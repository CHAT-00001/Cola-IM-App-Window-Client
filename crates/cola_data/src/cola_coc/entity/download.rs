// cola_data/src/cola_coc/entity/download.rs -- 数据 - COC - entity - 下载记录表
// 2026/8/4 20:53 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 下载记录表
/// * `pg schema`: `cola_coc`
/// * `table name`: `download`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CosDownloadEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub content: String,                   // 内容
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 不喜欢数量
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 是否删除
    pub add_time: i64,                     // 添加时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 常量字段
pub const COC_DOWNLOAD_COLUMNS: &str = r#"
    id, _id, user_id, video_id, content,
    likes, dislikes, visibility,
    region_code, status, is_deleted,
    add_time, upd_time, created_at, updated_at, deleted_at
"#;

//////// END
