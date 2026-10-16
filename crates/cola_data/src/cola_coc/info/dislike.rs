// data/src/cola_coc/info/dislike.rs -- 数据 - COC - info - 不喜欢记录信息
// 2026/8/9 21:58 Created.

////////

use crate::video::entity::dislike::VideoDislikeEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 不喜欢信息
/// * `desc`: `安全的信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoDislikeInfo {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub status: i16,                       // 状态: 0. 失效 1. 有效
    pub add_time: Option<i64>,             // 创建时间（兼容PHP旧版）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # 构造不喜欢信息
impl VideoDislikeInfo {
    //

    ////////

    /// 1. [BUILD] - 空的
    /// `desc`: `专门用于返回空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            user_id: 0,
            video_id: 0,
            status: 0,
            add_time: None,
            created_at: None,
            updated_at: None,
        }
    }

    ////////

    /// 2. [FROM] - 转换
    /// `desc`: `纯净的从数据库实体转换为不喜欢域模型`
    pub fn from_entity(entity: VideoDislikeEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            status: entity.status,
            add_time: entity.add_time,
            created_at: Option::from(entity.created_at),
            updated_at: entity.updated_at,
        }
    }
}

//////// END
