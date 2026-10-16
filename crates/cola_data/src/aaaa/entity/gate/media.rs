// data/src/cola_fs/entity/media.rs  -- 仓储 - FS - entity - 媒体表
// 2026/8/14 11:51 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

////////

/// # [ENTITY] - 文件服务 - 媒体资源实体（支持 LivePhoto、多清晰度 HLS 及封面管理）
/// * `pg schema`: `cola_fs` -- PG 模式
/// * `table name`: `media`  -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub app_id: Option<String>,            // 所属应用/模块标识
    pub media_type: i16, // 媒体类型: 1-普通图片, 2-普通视频, 3-LivePhoto(图+视频), 4-HLS多清晰度流
    pub status: i16,     // 状态码: 0-转码/处理中(草稿), 1-正常可用, -1-转码失败/禁用
    pub cover_file_id: Option<i64>, // 封面图文件 ID (视频/HLS/LivePhoto 的封面)
    pub main_file_id: Option<i64>, // 主文件 ID (LivePhoto 的静态图文件 / 普通图片/视频文件 ID)
    pub aux_file_id: Option<i64>, // 辅助文件 ID (LivePhoto 的配套短视频文件 ID)
    pub hls_playlist_url: Option<String>, // HLS 主播放列表地址 (.m3u8，针对 media_type = 4)
    pub variants_meta: Option<String>, // 多清晰度变体元数据 (JSON格式，存储不同分辨率如 1080p/720p 的路径、码率等)
    pub duration: Option<i32>,         // 媒体时长 (秒，适用于音视频)
    pub width: Option<i32>,            // 媒体宽度 (像素)
    pub height: Option<i32>,           // 媒体高度 (像素)
    pub is_deleted: Option<bool>,      // 逻辑删除
    pub create_time: i64,              // 创建时间（兼容旧版PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
pub const MEDIA_COLUMNS: &str = r#"
    id, _id, app_id, media_type, status, cover_file_id,
    main_file_id, aux_file_id, hls_playlist_url, variants_meta,
    duration, width, height, is_deleted, create_time,
    created_at, updated_at, deleted_at
"#;

////////
