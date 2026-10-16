// /shared.rs  -- 
// 2026/5/20 21:23 by wx: cestbon10080
// *
// * --------

////////

use serde::{Deserialize, Serialize};


////////

/// # [EVENT] - 分享事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSharedEvent {
    pub user_id: i64,
    pub video_id: i64,
    pub timestamp: i64,
}



// * --------
//////// END