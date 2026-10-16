// cola_data/src/video/info/collect.rs
// 🗄 数据 - ▶ 可乐视频 - info - 收藏
// 2026/5/20 19:30 Created.

////////

use crate::video::entity::collect::VideoCollectEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 收藏
/// * `desc`: `安全的视频收藏信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoCollectInfo {
    pub id: i64,                           // 收藏记录 ID
    pub uid: i64,                          // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub folder_id: i64,                    // 收藏夹 ID
    pub channel_id: i16,                   // 通道 ID
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态：0失效 1有效
    pub add_time: i32,                     // 添加时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # 构造收藏信息
impl VideoCollectInfo {
    /// 1. 专门用于返回“收藏不存在”的空对象
    pub fn empty() -> Self {
        Self {
            id: 0,
            uid: 0,
            video_id: 0,
            folder_id: 0,
            channel_id: 0,
            remark: None,
            status: 0,
            add_time: 0,
            created_at: None,
            updated_at: None,
        }
    }

    /// 2. 纯净的从数据库实体转换为收藏域模型
    pub fn from_entity(entity: VideoCollectEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            video_id: entity.video_id,
            folder_id: entity.folder_id,
            channel_id: entity.channel_id,
            remark: entity.remark,
            status: entity.status,
            add_time: entity.add_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//////// END
