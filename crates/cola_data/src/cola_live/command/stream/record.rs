// cola_data/src/cola_live/cmd/stream/record.rs
// 数据 - LIVE - cmd - 直播场次记录
// 2026/8/21 09:10 Created.

////////

use crate::cola_live::entity::stream::stream_record::LiveStreamRecordEntity;
use crate::cola_live::utils::record::build_stream_name;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # 1. [COMMAND] - 直播场次记录创建命令
/// * `desc`: `仅承载开播业务输入，uid 由服务端可信会话注入`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LiveRecordCommand {
    pub room_id: i64,                // 直播间主键
    pub show_id: Option<i64>,        // 业务场次标识
    pub title: Option<String>,       // 本场标题
    pub province: Option<String>,    // 省份
    pub city: Option<String>,        // 城市
    pub address: Option<String>,     // 详细地址
    pub lng: Option<f64>,            // 经度
    pub lat: Option<f64>,            // 纬度
    pub thumb: Option<String>,       // 封面
    pub channel_id: Option<i32>,     // 直播分类 ID
    pub live_type: Option<String>,   // 房间类型
    pub type_val: Option<String>,    // 房间类型值
    pub anyway: Option<i16>,         // 屏幕方向
    pub device_info: Option<String>, // 设备信息
    pub voice_type: Option<String>,  // 房间类型: 0语音 1视频
}

////////

impl LiveRecordCommand {
    /// # 1. [COMMAND] - 构造开播记录实体
    /// * `desc`: `生成场次标识、流名、推流地址和拉流地址`
    pub fn into_entity(
        self,
        uid: i64,
        push_domain: &str,
        play_flv_domain: &str,
        play_hls_domain: &str,
    ) -> LiveStreamRecordEntity {
        let now = Utc::now();
        let stream_name = build_stream_name(uid, self.room_id);
        let show_id = self.show_id.unwrap_or_else(|| now.timestamp_millis());

        LiveStreamRecordEntity {
            id: 0,
            _id: Some(uuid::Uuid::new_v4().simple().to_string()),
            uid,
            room_id: self.room_id,
            show_id,
            live_type: self.live_type.or(Some("0".to_string())),
            nums: 0,
            title: self.title,
            province: self.province,
            city: self.city,
            thumb: self.thumb,
            pull: Some(format!(
                "https://{}/live/{}.flv",
                play_flv_domain, stream_name
            )),
            stream: Some(stream_name.clone()),
            channel_id: self.channel_id,
            push_url: format!("rtmp://{}/cola_live/{}", push_domain, stream_name),
            pull_flv: Some(1),
            pull_hls: Some(1),
            is_mic: Some(0),
            is_hot: Some(0),
            is_recommend: Some(0),
            likes: 0,
            recommends: 0,
            is_off: Some(0),
            anyway: self.anyway.or(Some(1)),
            pk_uid: None,
            pk_stream: None,
            video_url: None,
            address: self.address,
            lng: self.lng,
            lat: self.lat,
            type_val: self.type_val,
            device_info: self.device_info,
            game_action: None,
            voice_type: self.voice_type.or(Some("1".to_string())),
            sw_player_status: Some(1),
            sw_player_id: None,
            sw_pull_url: Some(format!(
                "https://{}/live/{}.m3u8",
                play_hls_domain, stream_name
            )),
            recommend_time: None,
            status: 1,
            is_deleted: Some(false),
            start_at: now,
            end_at: None,
            deleted_at: None,
        }
    }
}

//////// END
