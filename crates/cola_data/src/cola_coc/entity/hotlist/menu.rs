// cola_data/src/cola_coc/entity/hotlist/menu.rs -- 数据 - COC - entity - 上热门 - 菜单
// 2026/8/2 14:00 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 运营中心 -  上热门菜单
/// * `pg schema`: `cola_coc`
/// * `table name`: `hotlist_menu`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CocHotlistMenuEntity {
    pub id: i16,                           // ID
    pub uid: i64,                          // 用户 ID
    pub name: String,                      // 名称
    pub name_zh: Option<String>,           // 中文名称
    pub pice_id: i32,                      // 价格
    pub exposure_qty: i32,                 // 有效值
    pub remark: Option<String>,            // 备注
    pub sort: i16,                         // 排序值 (默认9999)
    pub status: i16,                       // 状态码: 0失效 1有效
    pub is_deleted: bool,                  // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [CONSTANT] - 推荐记录表字段常量定义
/// * `desc`: `严格与 CocHotlistMenuEntity 结构体字段顺序和名称对齐`
pub const COC_HOTLIST_MENU_COLUMNS: &str = "\
    id, uid, name, name_zh, pice_id, exposure_qty, \
    remark, sort, status, is_deleted, \
    created_at, updated_at, deleted_at\
    ";

//////// END
