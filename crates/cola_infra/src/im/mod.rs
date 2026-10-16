// src/im/mod.rs -- Cola IM domain facade
// 2026/10/11 07:17 Created.

////////

// 当前先通过门面重导出既有实现，后续可在不改变外部 API 的情况下逐步迁移文件。
pub mod api {
    pub use crate::ui_api::{ContactsApi, MessagesApi};
}

pub mod client {
    pub use crate::client::{ClientEvent, ColaClient};
}

pub mod domain {
    pub use crate::network::domain::{
        AccountContext, ContactDeltaBatch, ContactRecord, ContactSyncResult, MessageRecord,
        OfflineMessageBatch, SyncApplyResult,
    };
}

pub mod event {
    pub use crate::client::ClientEvent as ImEvent;
}

pub use api::{ContactsApi, MessagesApi};
pub use client::{ClientEvent, ColaClient};
pub use domain::{
    AccountContext, ContactDeltaBatch, ContactRecord, ContactSyncResult, MessageRecord,
    OfflineMessageBatch, SyncApplyResult,
};
pub use event::ImEvent;

//////// END
