// cola_data/src/music/cmd/user.rs -- 数据 - MUSIC - cmd - 用户
// 2026/8/31 00:37 Created.

////////
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 可乐音乐 - 音乐用户创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicUserCreateCommand {
    pub user_id: i64,                      // 用户 ID
    pub _id: Option<String>,               // UUID v4
    pub name: String,                      // 名称
    pub name_zh: String,                   // 中文名称
    pub is_artist: Option<bool>,           // 是艺术家
    pub avatar_url: Option<String>,        // 作者封面
    pub description: String,               // 简介
    pub music_count: i64,                  // 作品数量
    pub collect_count: i64,                // 收藏数量
    pub album_count: i64,                  // 专辑数量
    pub favorites_count: i64,              // 最喜欢的数量
    pub status: i16,                       // 状态码
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

impl MusicUserCreateCommand {
    //

    ////////

    /// # [BUILDER] - 新的
    /// * `desc`: `推荐：提供一个带核心参数的 new 方法`
    pub fn new(user_id: i64, name: impl Into<String>) -> Self {
        let now = Utc::now();
        let name_str = name.into();
        Self {
            user_id,
            _id: None,
            name: name_str.clone(),
            name_zh: name_str, // 默认中文名和名一致，或留空
            is_artist: Some(false),
            avatar_url: None,
            description: String::new(),
            music_count: 0,
            collect_count: 0,
            album_count: 0,
            favorites_count: 0,
            status: 1, // 比如默认正常状态 1（根据你的业务定）
            is_deleted: Some(false),
            created_at: now,
            updated_at: now,
            deleted_at: None, // 新建时设为 None
        }
    }
}

impl Default for MusicUserCreateCommand {
    fn default() -> Self {
        // 如果有默认的系统用户 ID 或写 0 均可
        Self::new(0, "")
    }
}

//////// END
