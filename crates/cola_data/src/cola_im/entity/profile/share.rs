// cola_data/src/cola_im/entity/profile/ticket
// 数据 - IM - entity - profile - 名片分享表
// 2026/5/20 19:56 Created.

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - IM - 名片 - 分享表
/// * `pg schema`: `cola_im`
/// * `table name`: `profile_card_share`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ImCardShareEntity {
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
