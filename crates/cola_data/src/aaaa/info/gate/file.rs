// cola_data/src/cola_fs/info/file.rs
// 🗄 数据 - FS - info - 文件缓存
// 2026/8/14 14:00 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 文件缓存信息
/// * `desc`: `文件的内存缓存模型（从 entity 转换而来）`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: i64,
    pub app_id: Option<String>,
    pub bucket: String,
    pub object_key: String,
    pub file_name: String,
    pub file_ext: String,
    pub mime_type: String,
    pub file_size: i64,
    pub is_public: bool,
    pub status: i16,                       // 1正常 0删除 (临时态为0，正式态为1)
    pub expired_at: Option<DateTime<Utc>>, // 过期时间（临时文件）
    pub created_at: Option<DateTime<Utc>>,
}

//////// END
