// data/src/music/info/like.rs
// 数据 - MUSIC - info - 点赞
// 2026/8/24 12:33 Created.

////////

use crate::music::entity::like::MusicLikeEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 音乐点赞信息
/// * `desc`: `点赞记录日志追踪使用，保留作为用户查看点赞记录的备用格式`
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MusicLikeInfo {
    pub id: i64,                           // 点赞 ID (对齐 Entity 的 i64)
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 用户 ID
    pub music_id: i64,                     // 音乐 ID
    pub remark: Option<String>,            // 备注
    pub description: Option<String>,       // 描述
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub add_time: i64,                     // 创建时间（兼容旧版PHP）
    pub upd_time: i64,                     // 更新时间（兼容旧版PHP）
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

// 构造实现
impl MusicLikeInfo {
    //

    ////////

    /// # 1. [BUILD] - 创建
    /// * `desc`: 构造标准的 MusicLikeInfo
    pub fn new(
        id: i64,
        uid: i64,
        music_id: i64,
        remark: Option<String>,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp();

        Self {
            id,
            _id: Some(uuid::Uuid::new_v4().to_string()),
            uid,
            music_id,
            remark,
            description,
            status: 1, // 默认正常状态
            is_deleted: Some(false),
            add_time: timestamp,
            upd_time: timestamp,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    ////////

    /// # 2. [FROM] - 从 Entity 构建 Info
    /// * `desc`: 直接将数据库点赞实体映射为 Info 领域元数据
    pub fn from_entity(entity: &MusicLikeEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id.clone(),
            uid: entity.uid,
            music_id: entity.music_id,
            remark: entity.remark.clone(),
            description: entity.description.clone(),
            status: entity.status.unwrap_or(1),
            is_deleted: entity.is_deleted,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }

    ////////

    /// # 3. [BUILD] - 异常兜底场景
    /// * `desc`: 当点赞记录未命中或失效时，返回安全的空响应
    pub fn empty() -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp();

        Self {
            id: 0,
            _id: None,
            uid: 0,
            music_id: 0,
            remark: Some("点赞记录不存在或已失效".to_string()),
            description: None,
            status: 0,
            is_deleted: Some(true),
            add_time: timestamp,
            upd_time: timestamp,
            created_at: now,
            updated_at: now,
            deleted_at: Some(now),
        }
    }
}

//////// END
