// cola_data/src/cola_live/info/category.rs
// 数据 - LIVE - info - 直播分类
// 2026/8/20 21:10 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # 1. [INFO] - 直播分类信息
/// * `desc`: `直播分类管理接口的统一返回对象`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveCategoryInfo {
    pub id: i64,             // 分类 ID
    pub uid: i64,            // 创建操作者 ID
    pub name: String,        // 中文名称
    pub name_en: String,     // 英文名称
    pub icon: String,        // 图标
    pub action_uid: i64,     // 最后操作者 ID
    pub sort: i16,           // 排序
    pub is_hot: i16,         // 是否热门
    pub is_recommended: i16, // 是否推荐
    pub status: i16,         // 状态: 0禁用 1启用
}

////////

impl From<crate::cola_live::entity::cate::class::LiveClassEntity> for LiveCategoryInfo {
    fn from(entity: crate::cola_live::entity::cate::class::LiveClassEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            name: entity.name,
            name_en: entity.name_en,
            icon: entity.icon,
            action_uid: entity.action_uid,
            sort: entity.sort,
            is_hot: entity.is_hot,
            is_recommended: entity.is_recommended,
            status: entity.status,
        }
    }
}

//////// END
