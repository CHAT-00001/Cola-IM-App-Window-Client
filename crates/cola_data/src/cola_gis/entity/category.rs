// video/src/cola_gis/entity/cola_fs.rs  -- 数据中心 -  GIS - Entity - 分类
// 2026/4/24 18:24 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

/// # [ENTITY] - 兴趣点 分类 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_category`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoClassifyEntity {
    pub name: String, // 视频分类名称
    pub name_en: String,
    pub description: String,
}

//////// END
