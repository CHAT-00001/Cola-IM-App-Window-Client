// data/src/video/entity/danmaku/danmaku.rs -- 数据 - VIDEO - entity - 弹幕记录表
// 2026-01-16 09:37:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 短视频 - 弹幕表
/// * `pg schema`: `video`
/// * `table name`: `danmaku`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct DanmakuEntity {
    pub id: i64,                           // 弹幕 ID
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub channel_id: i32,                   // 通道 ID
    pub content: String,                   // // 内容
    pub likes: i32,                        // 点赞数量
    pub dislikes: i32,                     // 不喜欢数量
    pub collects: i32,                     // 收藏数量
    pub visibility: i16,                   // 可见性
    pub color: Option<String>,             // 颜色（默认 #FFFFFF）
    pub mode: i16,                         // 模式: 0. 固定 1. 滚动
    pub play_time: i32,                    // 播放器规定时间（ms）
    pub duration: i16,                     // 弹幕留存时间（固定字幕时生效）
    pub status: i16,                       // 状态
    pub send_time: i64,                    // 发送时间（客户端生成）
    pub sync_time: i64,                    // 服务器同步时间
    pub is_deleted: Option<bool>,          // 是否删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 软删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_DANMAKU_COLUMNS: &str = r#"
    id, _id, user_id, video_id, channel_id, content, likes,
    dislikes, collects, visibility, color, mode, play_time, duration,
    status, send_time, sync_time, is_deleted, created_at,
    updated_at, deleted_at
"#;

//////// END
