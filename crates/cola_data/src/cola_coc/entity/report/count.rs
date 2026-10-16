// cola_data/src/new/handler/count  -- 据 - USER - Entity - 短视频用户计数表
// 2026/5/22 18:46 by wx: cestbon10080
// * --------
// * --------

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频 - 用户权限
/// * table name: video_user_count
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoUserCountEntity {
    pub user_id: i64, // 用户 ID
    // -- 主动
    pub by_post_count: i32,    // 我的视频数量
    pub by_view_count: i32,    // 我看过的视频数量
    pub by_like_count: i32,    // 我点赞的视频数量
    pub by_collect_count: i32, // 我收藏的视频数量
    pub by_share_count: i32,   // 我分享的数量
    pub by_report_count: i32,  // 我举报的数量
    // -- 被动
    pub at_likes_count: i32,    // 被点赞数量
    pub at_collects_count: i32, // 被收藏数量
    pub at_view_count: i32,     // 被浏览数量

    pub level: Option<i16>,        // 用户等级
    pub author_level: Option<i16>, // 主播等级
    // -- 来源 --
    pub lat: Option<f64>,              // 纬度
    pub lng: Option<f64>,              // 经度
    pub login_ip: std::net::IpAddr,    // 当前登录IP
    pub register_ip: std::net::IpAddr, // 注册IP
    // -- 状态 --
    pub status: Option<i16>, // 状态
    // -- time --
    pub add_time: i32,            // 创建时间（兼容PHP程序）
    pub sync_time: Option<i64>,   // 同步时间
    pub create_time: Option<i32>, // 创建时间戳（UTC）
    pub update_time: Option<i32>, // 修改时间 （UTC）
}
