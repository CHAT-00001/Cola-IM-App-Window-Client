// cola_data/src/cola_live/info/record.rs
// 数据 - LIVE - info - 直播场次记录
// 2026/8/21 09:30 Created.

////////

use crate::cola_live::entity::stream::stream_record::LiveStreamRecordEntity;
use serde::{Deserialize, Serialize};

////////

/// # 1. [INFO] - 直播场次信息
/// * `desc`: `面向业务层的安全直播场次数据`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveRecordInfo {
    pub id: i64,                 // 场次 ID
    pub uid: i64,                // 主播 ID
    pub room_id: i64,            // 直播间 ID
    pub show_id: i64,            // 场次业务 ID
    pub title: Option<String>,   // 标题
    pub thumb: Option<String>,   // 封面
    pub channel_id: Option<i32>, // 分类 ID
    pub stream: Option<String>,  // 流名称
    pub push_url: String,        // 推流地址
    pub pull: Option<String>,    // FLV 播放地址
    pub hls_url: Option<String>, // HLS 播放地址
    pub status: i16,             // 状态: 0关闭 1直播中
    pub start_at: i64,           // 开播时间戳
    pub end_at: Option<i64>,     // 停播时间戳
    pub likes: i64,              // 点赞数
    pub recommends: i64,         // 推荐数
}

////////

impl From<LiveStreamRecordEntity> for LiveRecordInfo {
    fn from(entity: LiveStreamRecordEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            room_id: entity.room_id,
            show_id: entity.show_id,
            title: entity.title,
            thumb: entity.thumb,
            channel_id: entity.channel_id,
            stream: entity.stream,
            push_url: entity.push_url,
            pull: entity.pull,
            hls_url: entity.sw_pull_url,
            status: entity.status,
            start_at: entity.start_at.timestamp(),
            end_at: entity.end_at.map(|value| value.timestamp()),
            likes: entity.likes,
            recommends: entity.recommends,
        }
    }
}

//////// END
