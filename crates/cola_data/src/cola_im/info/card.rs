// cola_data/src/cola_im/info/card.rs
// 🗄 数据 - ⏹ 可乐IM - info - 名片
// 2026-07-07

use crate::cola_im::entity::contacts::contact::ImContactEntity as ContactCardEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 名片信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardInfo {
    pub id: i64,
    pub user_id: i64,
    pub contact_type: i16,
    pub content: String,
    pub first_name: String,
    pub last_name: String,
    pub visibility: i16,
    pub status: i16,
    pub add_time: i64,
    pub upd_time: i64,
}

impl CardInfo {
    pub fn from_entity(entity: ContactCardEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.owner_id,
            contact_type: 1,
            content: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            visibility: 1,
            status: 1,
            add_time: 0,
            upd_time: 0,
        }
    }
}

//////// END
