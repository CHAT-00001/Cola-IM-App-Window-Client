// src/storage/mod.rs -- Account-isolated storage facade
// 2026/10/9 00:20 Created.

////////

pub mod sqlite; // SQLite 仓储 (验证码等)
pub(crate) mod entity; // 数据库实体 (仅 crate 内可见)
pub(crate) mod migration; // 版本化迁移

use std::collections::HashMap;

use crate::network::domain::{
    AccountContext, ContactDeltaBatch, ContactRecord, ContactSyncResult, MessageRecord,
    OfflineMessageBatch, SyncApplyResult,
};
use crate::error::{Result, SdkError};

#[derive(Debug, Clone, Default)]
struct AccountStore {
    messages: HashMap<String, MessageRecord>,
    contacts: HashMap<String, ContactRecord>,
    cursors: HashMap<String, Option<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryStore {
    accounts: HashMap<(String, String), AccountStore>,
}

impl InMemoryStore {
    /// # [HANDLER] - InMemoryStore::apply_contact_delta
    /// * `DESC`: `按账号分表原子写入联系人增量并推进 contact 游标`
    pub fn apply_contact_delta(
        &mut self,
        context: &AccountContext,
        batch: &ContactDeltaBatch,
    ) -> Result<ContactSyncResult> {
        if context.application_id != batch.application_id || context.account_id != batch.account_id
        {
            return Err(SdkError::InvalidContext(
                "contact batch does not belong to the active account".to_string(),
            ));
        }
        if batch.biz_id != "contact" {
            return Err(SdkError::InvalidBatch(
                "contact delta must use biz_id=contact".to_string(),
            ));
        }

        let key = (context.application_id.clone(), context.account_id.clone());
        let current = self.accounts.get(&key).cloned().unwrap_or_default();
        let current_cursor = current.cursors.get("contact").cloned().flatten();
        if current_cursor != batch.previous_cursor {
            return Err(SdkError::CursorMismatch);
        }

        let mut next = current;
        let mut applied_count = 0;
        let mut deleted_count = 0;
        for contact in &batch.contacts {
            let should_apply = next
                .contacts
                .get(&contact.contact_id)
                .map_or(true, |old| contact.server_version > old.server_version);
            if !should_apply {
                continue;
            }
            if contact.deleted {
                deleted_count += 1;
            }
            next.contacts
                .insert(contact.contact_id.clone(), contact.clone());
            applied_count += 1;
        }
        next.cursors
            .insert("contact".to_string(), batch.next_cursor.clone());
        self.accounts.insert(key, next);

        Ok(ContactSyncResult {
            applied_count,
            deleted_count,
            next_cursor: batch.next_cursor.clone(),
            has_more: batch.has_more,
        })
    }

    /// # [HANDLER] - InMemoryStore::list_contacts
    /// * `DESC`: `读取当前账号分表中未删除的联系人`
    pub fn list_contacts(&self, context: &AccountContext) -> Vec<ContactRecord> {
        self.accounts
            .get(&(context.application_id.clone(), context.account_id.clone()))
            .map(|account| {
                account
                    .contacts
                    .values()
                    .filter(|contact| !contact.deleted)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// # [HANDLER] - InMemoryStore::apply_offline_batch
    /// * `DESC`: `按账号分表原子写入离线消息并推进同步游标`
    pub fn apply_offline_batch(
        &mut self,
        context: &AccountContext,
        batch: &OfflineMessageBatch,
    ) -> Result<SyncApplyResult> {
        if context.application_id != batch.application_id || context.account_id != batch.account_id
        {
            return Err(SdkError::InvalidContext(
                "offline batch does not belong to the active account".to_string(),
            ));
        }
        if batch.biz_id != "message" {
            return Err(SdkError::InvalidBatch(
                "offline message batch must use biz_id=message".to_string(),
            ));
        }

        let key = (context.application_id.clone(), context.account_id.clone());
        let current = self.accounts.get(&key).cloned().unwrap_or_default();
        let current_cursor = current.cursors.get(&batch.biz_id).cloned().flatten();
        if current_cursor != batch.previous_cursor {
            return Err(SdkError::CursorMismatch);
        }

        let mut next = current;
        let mut applied_count = 0;
        let mut deleted_count = 0;
        for message in &batch.messages {
            let should_apply = next
                .messages
                .get(&message.message_id)
                .map_or(true, |old| message.server_version > old.server_version);
            if !should_apply {
                continue;
            }
            if message.deleted {
                deleted_count += 1;
            }
            next.messages
                .insert(message.message_id.clone(), message.clone());
            applied_count += 1;
        }
        next.cursors
            .insert(batch.biz_id.clone(), batch.next_cursor.clone());
        self.accounts.insert(key, next);

        Ok(SyncApplyResult {
            applied_count,
            deleted_count,
            next_cursor: batch.next_cursor.clone(),
            has_more: batch.has_more,
        })
    }

    /// # [HANDLER] - InMemoryStore::delete_local_message
    /// * `DESC`: `仅删除当前账号分表中的本地消息`
    pub fn delete_local_message(
        &mut self,
        context: &AccountContext,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<()> {
        let key = (context.application_id.clone(), context.account_id.clone());
        let account = self
            .accounts
            .get_mut(&key)
            .ok_or_else(|| SdkError::NotFound(message_id.to_string()))?;
        let message = account
            .messages
            .get_mut(message_id)
            .ok_or_else(|| SdkError::NotFound(message_id.to_string()))?;
        if message.conversation_id != conversation_id {
            return Err(SdkError::NotFound(message_id.to_string()));
        }
        account.messages.remove(message_id);
        Ok(())
    }

    /// # [HANDLER] - InMemoryStore::list_messages
    /// * `DESC`: `读取当前账号分表中的消息`
    pub fn list_messages(&self, context: &AccountContext) -> Vec<MessageRecord> {
        self.accounts
            .get(&(context.application_id.clone(), context.account_id.clone()))
            .map(|account| account.messages.values().cloned().collect())
            .unwrap_or_default()
    }

    /// # [HANDLER] - InMemoryStore::cursor
    /// * `DESC`: `读取当前账号指定业务的同步游标`
    pub fn cursor(&self, context: &AccountContext, biz_id: &str) -> Option<String> {
        self.accounts
            .get(&(context.application_id.clone(), context.account_id.clone()))
            .and_then(|account| account.cursors.get(biz_id).cloned().flatten())
    }
}

//////// END
