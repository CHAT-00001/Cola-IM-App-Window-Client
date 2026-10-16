// cola_data/src/cola_fs/cmd/upload.rs
// 🗄️ 数据 - FS - 通用 UGC 上传命令
// 2026/8/17 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 通用上传文件
/// * `desc`: `业务模块只提交文件元数据，Bucket 和 Object Key 由 FS 服务端决定`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileRequest {
    pub role: String,              // 业务文件角色
    pub file_name: String,         // 原始文件名
    pub mime_type: String,         // MIME 类型
    pub file_size: i64,            // 文件大小(Byte)
    pub file_hash: Option<String>, // 文件 Hash
}

////////

/// # [COMMAND] - 通用上传会话
/// * `desc`: `app_id 由业务 CASE 固定注入，不信任客户端值`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUploadSessionRequest {
    pub app_id: String,                // 业务应用标识
    pub ugc_type: String,              // UGC 类型
    pub idempotency_key: String,       // 幂等键
    pub files: Vec<UploadFileRequest>, // 文件列表
}

////////

/// # [INFO] - 通用上传策略
/// * `desc`: `按 app_id 限制角色、MIME、数量和单文件大小`
#[derive(Debug, Clone)]
pub struct UploadPolicy {
    pub app_id: &'static str,
    pub ugc_type: &'static str,
    pub max_files: usize,
    pub allowed_mime_types: &'static [&'static str],
}

impl UploadPolicy {
    ////////

    /// # 1. [POLICY] - 根据 app_id 获取上传策略
    /// * `desc`: `所有 UGC 业务共用的服务端策略`
    pub fn by_app_id(app_id: &str) -> Option<Self> {
        match app_id {
            "ugc-video" => Some(Self {
                app_id: "ugc-video",
                ugc_type: "video",
                max_files: 2,
                allowed_mime_types: &["image/", "video/"],
            }),
            "ugc-avatar" => Some(Self {
                app_id: "ugc-avatar",
                ugc_type: "avatar",
                max_files: 1,
                allowed_mime_types: &["image/jpeg", "image/png"],
            }),
            "ugc-background" => Some(Self {
                app_id: "ugc-background",
                ugc_type: "background",
                max_files: 1,
                allowed_mime_types: &["image/jpeg", "image/png"],
            }),
            "ugc-post" => Some(Self {
                app_id: "ugc-post",
                ugc_type: "post",
                max_files: 20,
                allowed_mime_types: &["image/", "video/", "audio/"],
            }),
            _ => None,
        }
    }

    ////////

    /// # 2. [POLICY] - 校验文件
    /// * `desc`: `校验角色数量、文件名和 MIME 类型`
    pub fn validate(&self, files: &[UploadFileRequest]) -> anyhow::Result<()> {
        if files.is_empty() || files.len() > self.max_files {
            return Err(anyhow::anyhow!(
                "{} 文件数量必须在 1 到 {} 之间",
                self.app_id,
                self.max_files
            ));
        }
        for file in files {
            if file.file_name.trim().is_empty()
                || file.file_name.contains('/')
                || file.file_name.contains('\\')
            {
                return Err(anyhow::anyhow!("文件名不合法: {}", file.role));
            }
            if file.file_size <= 0 {
                return Err(anyhow::anyhow!("文件大小必须大于 0: {}", file.role));
            }
            if !self.allowed_mime_types.iter().any(|allowed| {
                if allowed.ends_with('/') {
                    file.mime_type.starts_with(allowed)
                } else {
                    file.mime_type == *allowed
                }
            }) {
                return Err(anyhow::anyhow!(
                    "{} 不允许上传 MIME 类型: {}",
                    self.app_id,
                    file.mime_type
                ));
            }
        }
        Ok(())
    }
}

//////// END
