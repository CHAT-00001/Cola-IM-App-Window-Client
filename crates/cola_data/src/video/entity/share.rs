// cola_data/src/video/entity/share.rs -- 数据 - VIDEO - entity - 分享记录表
// 2026/5/20 19:56

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 分享表
/// * `pg schema`: `video` -- PG模式
/// * `table name`: `shares` -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoShareEntity {
    pub id: i64,              // 分享 ID
    pub user_id: i64,         // 谁分享的
    pub video_id: i64,        // 分享了哪个视频
    pub target_platform: i16, // 分享到了哪里：1. 微信 2. 朋友圈 3. QQ 4. 复制链接
    pub share_code: String,   // 💡 自动生成的唯一分享口令/短码
    pub sync_id: String,      // 同步 id （客户端生成 uuid v4）
    pub sync_time: i64,       // 同步时间
    pub create_time: i64,     // 创建时间 （客户端生成）
}

//////// END
