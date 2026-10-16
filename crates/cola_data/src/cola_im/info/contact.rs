// cola_data/src/cola_im/info/contact.rs  -- IM - info - 联系人
// 2026-07-07

//////

use crate::cola_im::entity::contacts::contact::ImContactEntity as ContactEntity;
use serde::{Deserialize, Serialize};

//////

/// # [INFO] - 联系人信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactInfo {
    pub id: i64,
    pub owner_id: i64,
    pub group_id: Option<i64>,
    pub card_id: i64,
    pub remark_name: Option<String>,
    pub is_stared: i16,
    pub favorites: bool,
    pub blocked: bool,
    pub add_time: i64,
    pub upd_time: i64,
}

impl ContactInfo {
    pub fn from_entity(entity: ContactEntity) -> Self {
        Self {
            id: entity.id,
            owner_id: entity.owner_id,
            group_id: entity.group_id,
            card_id: entity.card_id,
            remark_name: entity.remark_name,
            is_stared: entity.is_stared,
            favorites: entity.favorites,
            blocked: entity.blocked,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
        }
    }
}

////// END
