// cola_data/src/cola_gis/entity/count  -- 数据中心 - GIS - entity - 收藏
// 2026/5/20 19:05 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - POI 收藏 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_collect`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct PoiCollectEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      //  用户 ID
    pub poi_id: i64,                       // 兴趣点 ID
    pub folder_id: i64,                    //
    pub channel: i16,                      // 频道
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub sync_time: i64,                    // 同步时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
}

//////// END
