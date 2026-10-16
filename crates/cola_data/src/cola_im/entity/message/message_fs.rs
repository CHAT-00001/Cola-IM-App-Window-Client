// cola_data/src/cola_im/entity/message/message_fs.rs
// 🗄 数据 - ⏹ 可乐IM - entity - message - 文件存储
// 2026/7/7 15:24

////////

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

////////

/// # [ENTITY] - 消息文件存储
/// * `pg schema`: `cola_im`
/// * `table name`: `message_fs`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageFileEntity {
    pub id: i64,                   // 用户ID
    pub name: String,              // 文件名称
    pub r#type: FileType,          // 文件类型(枚举)
    pub file_url: Option<String>,  // 文件链接
    pub cover_url: Option<String>, // 视频封面链接(livephoto时可选)
    pub video_url: Option<String>, // 视频链接(可选)
    pub s3_id: i16,                // S3配置ID (关联第三方配置表中的存储配置)
    pub s3_key: String,            // S3对象键 (文件在Bucket中的路径)
    pub size: i32,                 // 文件尺寸(KB)
    pub width: Option<i16>,        // 帧宽度(可选)
    pub height: Option<i16>,       // 帧高度(可选)
    pub fps: Option<i16>,          // 帧数/share (可选, 不使用浮点数)
    pub duration: Option<i16>,     // 时长(可选)
    pub bit: Option<i16>,          // 色深(可选)
    pub status: i16,               // 状态
    pub uploaded_at: i64,          // 上传时间 - 机器
    pub deleted_at: Option<i64>,   // 删除时间 - 机器
}

/// # 文件类型枚举
/// 支持消息中常见的媒体文件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    /// 照片/图片
    Image,
    /// 语音消息
    Voice,
    /// 视频文件
    Video,
    /// 实况照片 (Live Photo)
    LivePhoto,
    /// 普通文件 (文档、压缩包等)
    File,
    /// 音频文件 (音乐等)
    Audio,
    /// GIF动图
    Gif,
}

// 为枚举实现 FromRow，使其能与数据库映射
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for FileType {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let type_str: String = row.try_get("type")?;
        match type_str.as_str() {
            "image" => Ok(FileType::Image),
            "voice" => Ok(FileType::Voice),
            "new" => Ok(FileType::Video),
            "live_photo" => Ok(FileType::LivePhoto),
            "file" => Ok(FileType::File),
            "audio" => Ok(FileType::Audio),
            "gif" => Ok(FileType::Gif),
            _ => Err(sqlx::Error::Decode("Unknown file type".into())),
        }
    }
}

// 为枚举实现 Display，方便转换为字符串存储到数据库
impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Image => write!(f, "image"),
            FileType::Voice => write!(f, "voice"),
            FileType::Video => write!(f, "new"),
            FileType::LivePhoto => write!(f, "live_photo"),
            FileType::File => write!(f, "file"),
            FileType::Audio => write!(f, "audio"),
            FileType::Gif => write!(f, "gif"),
        }
    }
}

// 使用时提取扩展名
impl MessageFileEntity {
    pub fn extension(&self) -> Option<&str> {
        self.name.rsplit('.').next()
    }

    ////////

    /// # 1. [CASE] - 图形扩展名
    pub fn is_image(&self) -> bool {
        matches!(
            self.extension(),
            Some("jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "avif" | "heic")
        )
    }

    ////////

    /// # 2. [CASE] - 视频扩展名
    pub fn is_video(&self) -> bool {
        matches!(self.extension(), Some("mp4" | "mov" | "flv" | "hls" | "ts"))
    }

    ////////

    /// # 3. [CASE] - 其他文件扩展名
    pub fn is_file(&self) -> bool {
        matches!(
            self.extension(),
            Some("doc" | "cadx" | "dwg" | "dxf" | "max")
        )
    }
}

//////// END
