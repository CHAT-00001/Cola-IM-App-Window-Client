// cola_data/src/cola_gis/entity/danmark.rs  -- 数据中心 - GIS - entity - 弹幕
// 2026-01-16 09:37

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 兴趣点 弹幕 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct PoiDanmakuEntity {
    pub id: i64,                 // 弹幕 ID
    pub send_id: Option<String>, // 发送 ID
    pub user_id: i64,            // 用户 ID
    pub poi_id: i64,             // PO ID
    pub channel_id: i16,         // 频道 ID
    pub content: String,         // 内容
    pub likes: i32,              // 点赞数量
    pub dislikes: i32,           // 讨厌数量
    pub visibility: i16,         // 可见性
    pub color: Option<String>,   // 弹幕颜色: 默认 #FFFFFF
    pub mode: i16,               // 模式: 0. 固定 1. 滚动
    pub play_time: i32,          // 播放器时间
    pub duration: i16,           // 存留时长
    pub send_time: i64,          // 发送事件
    pub sync_time: i64,          // 同步时间
    pub status: i16,             // 状态
}

////////

/// # [COLUMNS] - 兴趣点 评论 数据表字段
pub const POI_DANMAKU_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, title, title_at_uids, description, desc_at_uids,
    thumb, thumb_s, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, steps, collects, comments,
    done_play_qty, visibility, allow_comment, allow_danmaku, shares,
    is_public, status, music_id, goods_id, addtime, created_at, updated_at
"#;

//////// END
