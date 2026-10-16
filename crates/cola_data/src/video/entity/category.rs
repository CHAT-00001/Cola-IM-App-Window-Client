// cola_data/src/new/entity/category.rs  --
// 🗄️ 数据 - ▶ 可乐视频 - entity - 分类
// 2026/4/24 18:24

////////

use serde::{Deserialize, Serialize};

////////

/// # [ENTITY] - 短视频 - 分类表
/// * `pg schema`: `video`
/// * `table name`: `category`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoClassifyEntity {
    pub id: i16,             // ID
    pub name: String,        // 视频分类名称
    pub name_en: String,     // 英文名称
    pub description: String, // 描述
    pub add_time: i64,       // 添加时间 - 机器
    pub upd_time: i64,       // 更新时间 - 机器
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_CLASSIFY_COLUMNS: &str = r#"
    id, name, name_en, description, add_time, upd_time
"#;

//////// END
