// cola_data/src/cola_gis/cmd/add  -- 数据中心 - GIS - Command - POI 评论
// 2026/5/20 12:01

////////

use crate::cola_gis::entity::comment::PoiCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum CommentType {
    Text = 1,
    Image = 2,
    LivePhoto = 3,
    Video = 4,
    Voice = 5,
    Other = 10,
}

impl TryFrom<i16> for CommentType {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CommentType::Text),
            2 => Ok(CommentType::Image),
            3 => Ok(CommentType::LivePhoto),
            4 => Ok(CommentType::Video),
            5 => Ok(CommentType::Voice),
            n if n >= 1 && n <= 10 => Ok(CommentType::Other),
            _ => Err("璇勮绫诲瀷蹇呴』鍦?1..10 涔嬮棿"),
        }
    }
}

/// # [COMMAND] - 兴趣点 评论 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiCommentCommand {
    pub user_id: i64,               // 用户 ID
    pub poi_id: i64,                // 兴趣点 ID
    pub parent_id: Option<i64>,     // 父评论 ID
    pub comment_type: i16,          // 评论类型 (1..10)
    pub content: String,            // 内容
    pub payload: MediaInfo,         // 负载
    pub photos_url: Option<String>, // 照片 URL
    pub video_url: Option<String>,  // 视频 URL
    pub voice_url: Option<String>,  // 语音 URL
    pub likes: i32,                 // 点赞数量
    pub dislikes: i32,              // 讨厌数量
    pub collects: i32,              // 收藏数量
    pub visibility: i16,            // 可见性
    pub add_time: i64,              // 添加时间 - 机器
    pub created_at: DateTime<Utc>,  // 创建时间 - 人类
    pub updated_at: DateTime<Utc>,  // 更新时间 - 人类
}

/// # [BUILD] - 构造
impl PoiCommentCommand {
    /// 灏嗗綋鍓嶇殑 Command 杞崲涓烘牳蹇冮鍩熺殑璇勮瀹炰綋 (Entity)
    /// 鍚屾椂浠庡閮紙鐢ㄤ緥/涓氬姟涓婁笅鏂囷級鍔ㄦ€佹敞鍏ュ疄鏃剁殑鐪熷疄 uid 鍜?video_id
    pub fn into_entity(self, real_uid: i64, real_video_id: i64) -> PoiCommentEntity {
        // 1. 鑾峰彇褰撳墠 UTC 鏃堕棿鎴?
        let now = Utc::now().timestamp();
        let now_ts = Utc::now();

        // 2. 鏍￠獙璇勮绫诲瀷 (1..10)锛屼笉鍚堟硶鍒欓粯璁ゅ瓨涓?1 (鏂囨湰)
        let validated_type = CommentType::try_from(self.comment_type)
            .map(|t| t as i16)
            .unwrap_or(1);

        // 3. 鍏煎鑰佺郴缁燂細鑷姩浠庡墠绔紶鏉ョ殑 payload 涓彁鍙栧浘鐗?URL 鐢ㄩ€楀彿鎷兼帴
        let legacy_photos_url = if self.payload.items.is_empty() {
            None
        } else {
            let urls: Vec<String> = self
                .payload
                .items
                .iter()
                .map(|item| match item {
                    MediaItem::Photo { url } => url.clone(),
                    MediaItem::LivePhoto { image_url, .. } => image_url.clone(),
                })
                .collect();
            Some(urls.join(",")) // 缁忓吀 PHP 鏃朵唬鐨勯€楀彿鍒嗛殧瀛樺偍
        };

        PoiCommentEntity {
            user_id: real_uid,     // 用户 ID
            poi_id: real_video_id, // poi ID
            parent_id: self.parent_id,
            comment_type: validated_type,
            content: self.content,
            // payload: self.payload,
            photos_url: legacy_photos_url,
            video_url: self.video_url,
            voice_url: self.voice_url,

            // 鏍稿績涓氬姟鍒濆榛樿鍊艰鍒?
            likes: 0,
            dislikes: 0,
            collects: 0,
            visibility: 4, // 榛樿鎵€鏈変汉鍙

            add_time: now,
            created_at: Option::from(now_ts),
            updated_at: Option::from(now_ts),
            ..Default::default()
        }
    }
}

/// 媒体项目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "cola_data")]
pub enum MediaItem {
    /// 鏅€氱収鐗囷細鍙渶浼犲叆鍥剧墖 URL
    Photo { url: String },
    /// 瀹炲喌鐓х墖锛氬寘鍚竴寮犻潤鎬佸浘鍜屼竴娈电煭瑙嗛
    LivePhoto {
        image_url: String,
        video_url: String,
    },
}

/// 媒体信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaInfo {
    pub items: Vec<MediaItem>,
}

impl MediaInfo {
    /// 渚挎嵎鏋勯€犲櫒锛氬垱寤轰竴涓┖鐨勫獟浣撹礋杞?
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 杩藉姞涓€涓獟浣撴枃浠?
    pub fn push(&mut self, item: MediaItem) {
        self.items.push(item);
    }
}

//////// END
