// cola_data/src/cola_gis/entity/count.rs  -- 可乐数据中心 - GIS - entity - count
// 2026/4/12 21:40 updated by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// # ENTITY - 用户资产统计
/// 独立于用户主表，承载高频更新的计数逻辑
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct UserCountEntity {
    pub id: i64,
    pub user_id: i64,

    // --- 内容发布资产 ---
    pub publish_dynamic_count: i32, // 动态总数
    pub publish_video_count: i32,   // 视频总数
    pub publish_line_count: i32,    // 流水线总数
    pub publish_stream_count: i32,  // 流总数

    // --- 互动行为（我发出的） ---
    pub add_liked_video_count: i32,    // 我点赞的视频数
    pub add_liked_dynamic_count: i32,  // 我点赞的动态数
    pub add_collect_video_count: i32,  // 我收藏的视频数
    pub add_collect_dynamic_count: i32,// 我收藏的动态数
    pub send_comment_count: i32,       // 我发的评论数
    pub send_danmaku_count: i32,       // 我发的弹幕数

    // --- 荣誉资产（我收到的/被动的） ---
    pub be_liked_count: i64,      // 总获赞数（视频+动态+评论等，高频重要）
    pub be_collect_count: i64,    // 总被收藏数
    pub reply_comment_count: i64, // 评论被回复数
    pub reply_danmaku_count: i64, // 弹幕被回复数

    // --- 社交关系 ---
    pub fans_count: i32,    // 粉丝数
    pub follow_count: i32,  // 关注数 (从 i16 改为 i32)
    pub friends_count: i32, // 好友数 (修正拼写，改为 i32)
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

//////// END