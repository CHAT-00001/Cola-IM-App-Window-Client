// cola_data/src/video/info/download.rs
// 🗄 数据 - ▶ 可乐视频 - info - 下载
// 2026/8/8 22:47 Created.

////////

use crate::video::entity::download::VideoDownloadEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 下载记录
/// * `desc`: `安全的视频下载记录信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadInfo {
    pub id: i64,                           // ID
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub content: String,                   // 内容
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 不喜欢数量
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间
    pub upd_time: i64,                     // 更新时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # 构造下载记录信息
impl DownloadInfo {
    //

    ////////

    /// # 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“下载记录不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            user_id: 0,
            video_id: 0,
            content: "下载记录不存在或已被删除".to_string(),
            likes: 0,
            dislikes: 0,
            visibility: 1,
            region_code: None,
            status: 0,
            add_time: 0,
            upd_time: 0,
            created_at: None,
            updated_at: None,
        }
    }

    ////////

    /// # 2. [FROM] - 转换
    /// * `desc`: `纯净的从数据库实体转换为下载记录域模型`
    pub fn from_entity(entity: VideoDownloadEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            content: entity.content,
            likes: entity.likes,
            dislikes: entity.dislikes,
            visibility: entity.visibility,
            region_code: entity.region_code,
            status: entity.status,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//////// END
