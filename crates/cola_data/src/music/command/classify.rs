// cola_data/src/music/cmd/classify.rs  -- 数据中心 - MUSIC - cmd - 分类
// 2026/7/7 13:08

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 分类 创建命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassifyCommand {
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
    pub sort: i16,           // 排序值: 默认9999
    pub status: i16,         // 状态码
    pub collect_count: i32,  // 收藏数量
    pub add_time: i64,       // 添加时间 - 机器
    pub upd_time: i64,       // 更新时间 - 机器
}

//////// END
