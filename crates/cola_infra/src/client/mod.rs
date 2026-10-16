// src/client/mod.rs -- Cola IM synchronization client
// 2026/10/9 00:20 Created.

////////

use crate::network::domain::{
    AccountContext, ContactDeltaBatch, ContactRecord, ContactSyncResult, OfflineMessageBatch,
    SyncApplyResult,
};
use crate::error::Result;
use crate::storage::InMemoryStore;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClientEvent {
    OfflineSyncStarted,
    OfflineMessagesSynced(SyncApplyResult),
    MessagesReceived {
        message_ids: Vec<String>,
        offline: bool,
    },
    LocalMessageDeleted {
        conversation_id: String,
        message_id: String,
    },
    SyncFailed(String),
    ContactSyncStarted,
    ContactsChanged {
        added_or_updated: Vec<String>,
        deleted: Vec<String>,
    },
    ContactSyncCompleted(ContactSyncResult),
}

#[derive(Debug, Clone)]
pub struct ColaClient {
    context: AccountContext,
    store: InMemoryStore,
    events: Vec<ClientEvent>,
}

impl ColaClient {
    /// # [HANDLER] - ColaClient::new
    /// * `DESC`: `创建绑定当前账号的 SDK 客户端`
    pub fn new(context: AccountContext) -> Self {
        Self {
            context,
            store: InMemoryStore::default(),
            events: Vec::new(),
        }
    }

    /// # [HANDLER] - ColaClient::sync_offline_messages
    /// * `DESC`: `原子应用离线消息批次并在提交后生成 UI 事件`
    pub fn sync_offline_messages(
        &mut self,
        batch: &OfflineMessageBatch,
    ) -> Result<SyncApplyResult> {
        self.events.push(ClientEvent::OfflineSyncStarted);
        match self.store.apply_offline_batch(&self.context, batch) {
            Ok(result) => {
                let ids: Vec<String> = batch
                    .messages
                    .iter()
                    .map(|item| item.message_id.clone())
                    .collect();
                self.events
                    .push(ClientEvent::OfflineMessagesSynced(result.clone()));
                if !ids.is_empty() {
                    self.events.push(ClientEvent::MessagesReceived {
                        message_ids: ids,
                        offline: true,
                    });
                }
                Ok(result)
            }
            Err(error) => {
                self.events
                    .push(ClientEvent::SyncFailed(format!("{error:?}")));
                Err(error)
            }
        }
    }

    /// # [HANDLER] - ColaClient::sync_contacts
    /// * `DESC`: `原子应用联系人增量批次并在提交后生成 UI 事件`
    pub fn sync_contacts(&mut self, batch: &ContactDeltaBatch) -> Result<ContactSyncResult> {
        self.events.push(ClientEvent::ContactSyncStarted);
        match self.store.apply_contact_delta(&self.context, batch) {
            Ok(result) => {
                let mut changed = Vec::new();
                let mut deleted = Vec::new();
                for contact in &batch.contacts {
                    if contact.deleted {
                        deleted.push(contact.contact_id.clone());
                    } else {
                        changed.push(contact.contact_id.clone());
                    }
                }
                self.events.push(ClientEvent::ContactsChanged {
                    added_or_updated: changed,
                    deleted,
                });
                self.events
                    .push(ClientEvent::ContactSyncCompleted(result.clone()));
                Ok(result)
            }
            Err(error) => {
                self.events
                    .push(ClientEvent::SyncFailed(format!("{error:?}")));
                Err(error)
            }
        }
    }

    /// # [HANDLER] - ColaClient::contacts
    /// * `DESC`: `返回当前账号已经提交的联系人快照`
    pub fn contacts(&self) -> Vec<ContactRecord> {
        self.store.list_contacts(&self.context)
    }

    /// # [HANDLER] - ColaClient::delete_local_message
    /// * `DESC`: `删除当前账号的本地消息并在成功后生成事件`
    pub fn delete_local_message(&mut self, conversation_id: &str, message_id: &str) -> Result<()> {
        self.store
            .delete_local_message(&self.context, conversation_id, message_id)?;
        self.events.push(ClientEvent::LocalMessageDeleted {
            conversation_id: conversation_id.to_string(),
            message_id: message_id.to_string(),
        });
        Ok(())
    }

    /// # [HANDLER] - ColaClient::messages
    /// * `DESC`: `返回当前账号已经提交的消息快照`
    pub fn messages(&self) -> Vec<crate::network::domain::MessageRecord> {
        self.store.list_messages(&self.context)
    }

    /// # [HANDLER] - ColaClient::drain_events
    /// * `DESC`: `取出并清空待分发的 UI 事件`
    pub fn drain_events(&mut self) -> Vec<ClientEvent> {
        std::mem::take(&mut self.events)
    }

    /// # [HANDLER] - ColaClient::pop_event
    /// * `DESC`: `取出一个待分发事件并保留其余事件`
    pub fn pop_event(&mut self) -> Option<ClientEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }
}

//////// END
//////// END
