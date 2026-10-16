// cola_data/src/cola_gis/info/danmaku.rs  -- 数据中心 - GIS - info - 弹幕
// 2026/5/22 16:10

////////

use crate::cola_gis::entity::danmaku::PoiDanmakuEntity;
use crate::cola_user::info::user::UserInfo;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - POI 弹幕
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoiDanmakuInfo {
    pub id: i64,                 // 弹幕 ID
    pub send_id: Option<String>, // 同步 ID
    pub user_id: i64,            // 用户 ID
    pub poi_id: i64,             // 兴趣点 ID
    pub content: String,         // 内容
    pub likes: i32,              // 点赞数量
    pub dislikes: i32,           // 讨厌数量
    pub color: String,           // 弹幕颜色 (默认"#FFFFFF")
    pub mode: i16,               // 模式 1.滚动 2.固定 3.
    pub play_time: i32,          // 播放器轨道时间
    pub duration: i16,           // 留存时长
    pub send_time: i64,          // 发送时间
    pub sync_time: i64,          // 同步时间
}

////////

/// # 构造
impl PoiDanmakuInfo {
    ////////

    /// # [FROM] - 实体来源
    pub fn from_entity(entity: PoiDanmakuEntity) -> Self {
        Self {
            id: entity.id,
            send_id: entity.send_id,
            user_id: entity.user_id,
            poi_id: entity.poi_id,
            content: entity.content,
            likes: entity.likes,
            dislikes: entity.dislikes,
            color: entity.color.unwrap_or_else(|| "#FFFFFF".to_string()),
            mode: entity.mode,
            play_time: entity.play_time,
            duration: entity.duration,
            send_time: entity.send_time,
            sync_time: entity.sync_time,
        }
    }

    /////////

    /// # [BUILD] - 新建
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        send_id: Option<String>,
        user_id: i64,
        poi_id: i64,
        content: String,
        likes: i32,
        dislikes: i32,
        color: Option<String>,
        mode: Option<i16>,
        play_time: i32,
        duration: Option<i16>,
        send_time: i64,
        sync_time: i64,
        video_author_id: i64,
    ) -> Self {
        let is_author = user_id == video_author_id;

        Self {
            id,
            send_id,
            user_id,
            poi_id,
            content,
            likes,
            dislikes,
            color: color.unwrap_or_else(|| "#FFFFFF".to_string()), // 弹幕颜色
            mode: mode.unwrap_or(0),
            play_time,
            duration: duration.unwrap_or(5000),
            send_time,
            sync_time,
        }
    }
}

//////// END
