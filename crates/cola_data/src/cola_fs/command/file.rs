// cola_data/src/fs/cmd/file.rs  -- 数据 - FS - cmd - 文件/对象命令
// 2026/8/14 12:30

////////

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

////////

////////

/// # [CMD] - 创建文件/对象（草稿态）参数载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileCmd {
    pub _id: Option<String>,
    pub app_id: Option<String>,
    #[serde(alias = "bucket")]
    pub bucket_key: String,
    pub object_key: String,
    #[serde(alias = "file_name")]
    pub original_name: Option<String>,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub file_hash: Option<String>,
    pub expired_at: Option<chrono::DateTime<chrono::Utc>>,
}

//////// END
