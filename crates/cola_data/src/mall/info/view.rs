// data/src/video/info/view.rs
// 🗄 数据 - ▶ 可乐视频 - info - 浏览信息
// 2026/8/4 20:42 Created.

////////

use crate::video::entity::view::VideoViewEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [VIEW INFO]
/// * `desc`: `视频浏览记录信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoViewInfo {
    pub id: i64,                           // ID
    pub _id: Option<String>,               // UUID v4
    pub user_id: i64,                      // 用户ID
    pub video_id: i64,                     // 视频ID
    pub time: i32,                         // 观看到的时间（秒）
    pub remark: Option<String>,            // 备注
    pub is_done: Option<bool>,             // 是否观看完毕
    pub count: Option<i16>,                // 观看次数
    pub addtime: i64,                      // 创建时间（兼容PHP旧版）
    pub updtime: Option<i64>,              // 更新时间（兼容PHP旧版）
    pub status: i16,                       // 状态 0. 失效  1. 正常
    pub is_deleted: Option<bool>,          // 是否删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

/// 构造浏览信息
impl VideoViewInfo {
    //

    ////////

    /// # 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“浏览记录不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            user_id: 0,
            video_id: 0,
            time: 0,
            remark: None,
            is_done: Some(false),
            count: Some(0),
            addtime: 0,
            updtime: None,
            status: 0,
            is_deleted: Some(false),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }

    ////////

    /// # 2. [FROM] - 转换
    /// * `desc`: `纯粹的从数据库实体转换`
    pub fn from_entity(entity: VideoViewEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            time: entity.time,
            remark: entity.remark,
            is_done: entity.is_done,
            count: entity.count,
            addtime: entity.addtime,
            updtime: entity.updtime,
            status: entity.status,
            is_deleted: entity.is_deleted,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

//////// END
