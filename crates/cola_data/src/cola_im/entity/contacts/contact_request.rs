// cola_data/src/cola_im/entity/contact_request.rs
// 🗄 数据 - ⏹ 可乐IM - entity - 联系人 添加请求
// 2026/7/7 15:53

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// #  [ENTITY] - 联系人 - 添加请求 实体表结构
/// * `pg schema`: `cola_im`
/// * `table name`: `contact_request`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ContactRequestEntity {
    pub id: i64,                 // 联系人 ID
    pub send_id: String,         // 发送 ID
    pub owner_id: i64,           // 用户 ID
    pub card_id: i64,            // 对方名片 ID
    pub message: Option<String>, // 验证消息
    pub is_refused: Option<i16>, // 是否拒绝
    pub dnd: bool,               // 勿扰
    pub blocked: bool,           // 是否已拉黑名单
    pub replay: Option<String>,  // 回复消息
    pub sort: Option<i64>,       // 排序
    pub status: i16,             // 状态码
    pub add_time: i64,           // 添加时间 - 机器
    pub upd_time: i64,           // 更新时间 - 机器
    pub del_time: i64,           // 删除时间 - 机器
}

//////// END
