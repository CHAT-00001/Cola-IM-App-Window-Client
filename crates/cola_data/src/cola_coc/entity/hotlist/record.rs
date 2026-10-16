// data/src/cola_coc/entity/hotlist/record.rs -- 数据 - COC - entity - hotlist - 记录表
// 2026/9/1 05:14 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 运营中心 - 上热门表
/// * `pg schema`: `cola_coc` - PG 模式
/// * `table name`: `hotlist_record` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct HotlistEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 用户 ID
    pub app_id: i16,                       // 应用 ID
    pub content_id: i64,                   // 内容 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub comment_type: i16,                 // 类型
    pub remark: Option<String>,            // 备注 (可选)
    pub exposure_qty: i32,                 // 加热数量
    pub order_id: Option<i64>,             // 订单 ID (可选)
    pub status: i16,                       // 状态
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 逻辑删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射，严格与 HotlistEntity 结构体对齐`
pub const COC_HOTLIST_COLUMNS: &str = r#"
    id, _id, uid, app_id, content_id, parent_id, comment_type,
    remark, exposure_qty, order_id, status, is_deleted,
    created_at, updated_at, deleted_at
"#;

//////// END
