// cola_data/src/cola_gis/entity/add  -- 数据中心 - GIS - entity - 分享
// 2026/5/20 19:56 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 兴趣点 收藏 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct PoiShareEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 操作者 ID
    pub poi_id: i64,                       // 兴趣点 ID
    pub target_platform: i16,              // 目标平台
    pub share_code: String,                // 分享码
    pub sync_id: String,                   // 同步 ID
    pub sync_time: i64,                    // 同步时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
}

//////// END
