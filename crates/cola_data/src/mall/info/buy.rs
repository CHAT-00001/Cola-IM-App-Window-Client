// data/src/video/info/buy.rs -- 数据 - VIDEO - info - 购买记录信息
// 2026/8/9 19:52 Created.

////////

use crate::video::entity::buy::VideoBuyEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 购买记录信息
/// * `desc`: `安全的视频购买信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoBuyInfo {
    pub id: i64,                           // 购买记录自增 ID
    pub uid: i64,                          // 购买发起者用户 ID
    pub video_id: i64,                     // 被购买的短视频 ID
    pub folder_id: i64,                    // 购买夹 ID：默认 0 (主购买夹/未分类)
    pub channel: i16,                      // 通道
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态：0失效 1有效
    pub add_time: i32,                     // 添加时间 (兼容旧版 PHP)
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

/// # 构造购买记录
impl VideoBuyInfo {
    /// 1. 专门用于返回“购买记录不存在”的空对象
    pub fn empty() -> Self {
        Self {
            id: 0,
            uid: 0,
            video_id: 0,
            folder_id: 0,
            channel: 0,
            remark: Some("购买记录不存在或已被删除".to_string()),
            status: 0,
            add_time: 0,
            is_deleted: Some(true),
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    /// 2. 纯净的从数据库实体转换为购买记录域模型
    pub fn from_entity(entity: VideoBuyEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            video_id: entity.video_id,
            folder_id: entity.folder_id,
            channel: entity.channel,
            remark: entity.remark,
            status: entity.status,
            add_time: entity.add_time,
            is_deleted: entity.is_deleted,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
