// cola_data/src/cola_im/info/contact_request.rs  -- IM - info - 联系人请求
// 2026-07-07

use crate::cola_im::entity::contacts::contact_request::ContactRequestEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactRequestInfo {
    pub id: i64,
    pub owner_id: i64,
    pub card_id: i64,
    pub message: Option<String>,
    pub is_refused: Option<i16>,
    pub replay: Option<String>,
    pub add_time: i64,
    pub upd_time: i64,
}

impl ContactRequestInfo {
    pub fn from_entity(entity: ContactRequestEntity) -> Self {
        Self {
            id: entity.id,
            owner_id: entity.owner_id,
            card_id: entity.card_id,
            message: entity.message,
            is_refused: entity.is_refused,
            replay: entity.replay,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
        }
    }
}
