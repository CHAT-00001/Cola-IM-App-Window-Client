// cola_data/src/video/info/hotlist.rs -- 数据 - VIDEO - info - 上热门信息
// 2026/5/21 00:58 Created.

////////

use crate::video::entity::hotlist::VideoHotlistEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 上热门信息
/// * `desc`: `安全的视频上热门记录信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoHotlistInfo {
    pub id: i64,                           // ID (自增 / 雪花)
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub remark: Option<String>,            // 备注 (可选)
    pub status: i16,                       // 状态码: 0失效 1有效
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub add_time: i64,                     // 添加时间(兼容旧版)
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

// 构造实现
impl VideoHotlistInfo {
    //

    ////////

    /// 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“上热门记录不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            user_id: 0,
            video_id: 0,
            remark: Some("上热门记录不存在或已被删除".to_string()),
            status: 0,
            is_deleted: Some(true),
            add_time: 0,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    /// 2. [FROM] - 转换
    /// * `desc`: `纯净的从数据库实体转换为上热门记录域模型`
    pub fn from_entity(entity: VideoHotlistEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            remark: entity.remark,
            status: entity.status,
            is_deleted: entity.is_deleted,
            add_time: entity.add_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
