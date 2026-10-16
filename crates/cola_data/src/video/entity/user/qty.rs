// repository/src/gate_grpc/qty.rs  -- 用户的视频统计

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// ## ENTITY - 用户社交资产统计（被动收到的）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAssetQty {
    pub uid: i64,             // 用户ID
    pub gained_likes: i64,    // 累计获赞总数 (别人点我的)
    pub gained_collects: i64, // 累计被收藏总数
    pub gained_views: i64,    // 视频总播放量
    pub fans_count: i64,      // 粉丝总数
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
