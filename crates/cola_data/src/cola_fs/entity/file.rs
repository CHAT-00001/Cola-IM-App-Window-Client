// cola_data/src/cola_fs/entity/file.rs  -- 数据中心 - FS - entity - 文件
// 2026/7/27 14:37

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 文件映射
/// * `pg schema`: `cola_fs` -- 文件存储
/// * `table name`: `file` -- S3 文件映射表
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct FsFileEntity {
    pub id: i64,                           // ID
    pub app_id: Option<String>,            // 所属应用/模块标识
    pub vendor_id: i64,                    // FK → three_vendor.id
    pub bucket: String,                    // 存储桶
    pub object_key: String,                // S3 Object Key（唯一）
    pub file_name: String,                 // 原始文件名
    pub file_ext: String,                  // 扩展名
    pub mime_type: String,                 // MIME 类型
    pub file_size: i64,                    // 文件大小(Byte)
    pub file_hash: String,                 // SHA256 / MD5
    pub is_public: bool,                   // 是否公开
    pub remark: Option<String>,            // 备注
    pub storage_class: Option<String>,     // STANDARD / IA / GLACIER...
    pub status: i16,                       // 1正常 0删除
    pub expired_at: Option<DateTime<Utc>>, // 过期时间（临时文件）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const FS_FILE_COLUMNS: &str = r#"
    id,
    vendor_id,
    bucket,
    object_key,
    file_name,
    file_ext,
    mime_type,
    file_size,
    file_hash,
    is_public,
    remark,
    storage_class,
    status,
    expired_at,
    created_at,
    updated_at
"#;

////////

impl FsFileEntity {
    ////////

    /// # 1. [ENTITY] - 转换为缓存信息
    /// * `desc`: `Entity → Info（用于缓存）`
    pub fn to_file_info(&self) -> anyhow::Result<crate::cola_fs::info::file::FileInfo> {
        use crate::cola_fs::info::file::FileInfo;

        Ok(FileInfo {
            id: self.id,
            app_id: self.app_id.clone(),
            bucket: self.bucket.clone(),
            object_key: self.object_key.clone(),
            file_name: self.file_name.clone(),
            file_ext: self.file_ext.clone(),
            mime_type: self.mime_type.clone(),
            file_size: self.file_size,
            is_public: self.is_public,
            status: self.status,
            expired_at: self.expired_at,
            created_at: self.created_at,
        })
    }
}

//////// END
