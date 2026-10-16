// cola_data/src/cola_gis/entity/get  -- 数据中心 - GIS - entity - 浏览记录
// 2026/3/28 05:56

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 兴趣点 浏览记录 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_visited`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoiViewEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 用户ID
    pub poi_id: i64,                       // 视频ID
    pub time: i32,                         // 观看到的时间
    pub remark: Option<String>,            // 备注
    pub add_time: i64,                     // 添加时间 - 机器 （兼容PHP旧版）
    pub status: i16,                       // 状态 0. 失效  1. 正常
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
}

//////// END
