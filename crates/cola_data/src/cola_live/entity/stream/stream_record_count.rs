// cola_live/src/cola_live/entity/live_record_count.rs  -- 数据中心 - LIVE - entity - 直播记录统计
// 2026/7/8 10:37

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播间 记录(场次) 统计表
/// * `pg schema`: `cola_live`
/// * `table name`: `stream_record_count`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamRecordCountEntity {
    pub record_id: String,            // 直播记录 ID
    pub likes: i64,                   // 点赞数量
    pub good_num: Option<i64>,        // 挂载商品数量
    pub hot_votes: Option<i64>,       // 当前热度值/票数
    pub gift_total_coin: Option<i64>, // 该场直播主播收入总金币
    pub gift_user_total: Option<i64>, // 该场直播观众消耗总金币
    pub banker_coin: Option<i64>,     // 庄家/游戏池金币
}

////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const LIVE_STREAM_RECORD_COUNT_COLUMNS: &str = r#"
    record_id, likes, good_num, hot_votes, gift_total_coin, gift_user_total, banker_coin
"#;
//////// END
