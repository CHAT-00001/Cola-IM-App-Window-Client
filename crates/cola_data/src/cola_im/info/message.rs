// cola_data/src/cola_im/info/message.rs  -- IM - info - 消息
// 2026-07-07

use crate::cola_im::entity::message::message::ImMessageEntity as MessageEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageInfo {
    pub id: i64,
    pub user_id: i64,
    pub at_id: i64,
    pub channel_id: i16,
    pub content: String,
    pub likes: i32,
    pub status: i16,
    pub send_time: i64,
    pub sync_time: i64,
}

impl MessageInfo {
    pub fn from_entity(entity: MessageEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            at_id: entity.at_id,
            channel_id: entity.channel_id,
            content: entity.content,
            likes: entity.likes,
            status: entity.status,
            send_time: entity.send_time,
            sync_time: entity.sync_time,
        }
    }
}
