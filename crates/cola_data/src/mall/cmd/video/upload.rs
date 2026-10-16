// cola_data/src/video/cmd/video/upload.rs
// 数据 - VIDEO - cmd - 通用上传会话
// 2026/8/17 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 短视频上传文件描述
/// * `desc`: `客户端只提交文件元数据，Bucket、Object Key 和所有权由服务端决定`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoUploadFileCmd {
    pub file_name: String,         // 客户端原始文件名
    pub mime_type: String,         // MIME 类型
    pub file_size: i64,            // 文件大小(Byte)
    pub file_hash: Option<String>, // 文件 Hash
}

////////

/// # [COMMAND] - 短视频上传会话申请
/// * `desc`: `一次申请封面和主视频两个物理文件的上传凭证`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoUploadSessionCmd {
    pub idempotency_key: String,   // 客户端幂等键
    pub cover: VideoUploadFileCmd, // 封面文件
    pub main: VideoUploadFileCmd,  // 主视频文件
}

//////// END
