// cola_data/src/cola_im/cmd/buy.rs
// 🗄️ 数据 - ✉️ 可乐IM - Command - 购买
// 2026/5/22 20:51 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 房间购买 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomBuyCommand {
    pub send_id: String,        // 发送 ID
    pub user_id: i64,           // 用户 ID
    pub album_id: Option<i64>,  // 专辑 ID
    pub video_id: i64,          // 视频 ID
    pub remark: Option<String>, // 备注
    pub qty: i32,               // 曝光量
    pub time_zone: String,      // 时间区间
    pub older: Option<String>,  // 订单 ID
}

//////// END
