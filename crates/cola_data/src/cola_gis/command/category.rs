// cola_data/src/cola_gis/cmd/cola_fs.rs  -- 可乐数据中心 - GIS - Command - 分类
// 2026/5/22 19:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - POI 分类 命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoiCategoryCommand {
    pub name: String,        // 当地名称
    pub name_en: String,     // 英文名称
    pub description: String, // 描述
    pub thumb: String,       // 缩略图
    pub sort: i16,           // 排序: 默认9999
}

//////// END
