// cola_data/src/music/cmd/music/new.rs  -- 数据中心 - MUSIC - cmd - 音乐
// 2026/5/22 16:28 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 可乐音乐 - 音乐创建命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusicCreateCommand {
    pub send_id: String,     // 发送 ID
    pub sync_id: String,     // 同步 ID
    pub name: String,        // 名称
    pub name_zh: String,     // 中文名称
    pub artist: String,      // 艺术家
    pub cover_url: String,   // 封面
    pub description: String, // 简介
    pub duration: i16,       // 时长（share）
    pub upload_type: i16,    // 1. 系统生成 2. 用户行为 3. 管理员
    pub use_count: i32,      // 被使用数量
    pub collect_count: i32,  // 收藏数量
    pub add_time: i64,       // 添加时间
    pub sync_time: i64,      // 服务器同步时间
}

////////

/// # [COMMAND] - 可乐音乐 - 音乐更新命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusicUpdateCommand {
    pub send_id: String,     // 发送 ID
    pub sync_id: String,     // 同步 ID
    pub name: String,        // 名称
    pub name_zh: String,     // 中文名称
    pub artist: String,      // 艺术家
    pub cover_url: String,   // 封面
    pub description: String, // 简介
    pub duration: i16,       // 时长（share）
    pub upload_type: i16,    // 1. 系统生成 2. 用户行为 3. 管理员
    pub use_count: i32,      // 被使用数量
    pub collect_count: i32,  // 收藏数量
    pub add_time: i64,       // 添加时间
    pub sync_time: i64,      // 服务器同步时间
}

//////// END
