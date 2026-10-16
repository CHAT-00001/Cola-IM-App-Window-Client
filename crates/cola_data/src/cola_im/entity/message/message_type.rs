// cola_data/src/cola_im/entity/message_type.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 消息类型
// 2026-07-07 14:00

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 消息类型
/// * `pg schema`: `cola_im`
/// * `table name`: `message_type`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageTypeEntity {
    pub id: i64,           // 用户ID
    pub name: String,      // 名称
    pub name_zh: String,   // 中文名称
    pub gained_views: i64, // 视频总播放量
    pub fans_count: i64,   // 粉丝总数
    pub updated_at: i64,
}

/// ## ENTITY - 用户个人行为统计（主动发出的）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserActionQty {
    pub uid: i64,           // 用户ID
    pub post_count: i64,    // 我发布的视频数
    pub like_count: i64,    // 我点赞别人的总数
    pub collect_count: i64, // 我收藏的总数
    pub follow_count: i64,  // 我关注的人数
    pub updated_at: i64,
}

//////// END
