// src/ui_api.rs -- UI-facing synchronization API facade
// 2026/10/11 00:30 Created.

////////

use serde_json::Value;

use crate::client::{ClientEvent, ColaClient};
use crate::error::Result;
use crate::handler::auth::code::send_captcha;
use crate::network::domain::{
    ContactDeltaBatch, ContactRecord, ContactSyncResult, OfflineMessageBatch, SyncApplyResult,
};
use crate::network::endpoint::EndpointRegistry;
use crate::network::http::HttpSender;
use crate::storage::sqlite::SqliteStore;

////////

/// # [UI] - 登录前认证接口门面 (进程内调用 / FFI 共用)
/// * `desc`: `持有接入点注册表、HTTP 发送器与 SQLite 仓储, 向 UI 暴露验证码等认证能力`
pub struct AuthApi<'a> {
    registry: &'a EndpointRegistry, // 接入点注册表
    sender: &'a HttpSender,         // HTTP 发送器
    store: &'a SqliteStore,         // SQLite 仓储
    application_id: &'a str,        // 应用 ID
}

impl<'a> AuthApi<'a> {
    /// # 1. [UI] - 创建认证接口门面
    /// * `desc`: `绑定接入点、发送器、存储与应用 ID 创建门面`
    pub fn new(
        registry: &'a EndpointRegistry, // 接入点注册表
        sender: &'a HttpSender,         // HTTP 发送器
        store: &'a SqliteStore,         // SQLite 仓储
        application_id: &'a str,        // 应用 ID
    ) -> Self {
        Self {
            registry,
            sender,
            store,
            application_id,
        }
    }

    /// # 2. [UI] - 发送验证码
    /// * `desc`: `发送验证码并返回服务端 JSON, 同时已落库供 UI 查询`
    pub fn send_captcha(
        &self,
        area: &str,     // 区号 "+86"
        phone_no: &str, // 手机号
    ) -> Result<Value> {
        send_captcha(
            self.registry,
            self.sender,
            self.store,
            self.application_id,
            area,
            phone_no,
        )
    }

    /// # 3. [UI] - 读取最近一次验证码响应
    /// * `desc`: `从本地 SQLite 读取最近一次验证码响应 JSON`
    /// * `condition`: `无记录时返回 None`
    pub fn last_captcha(&self, area: &str, phone_no: &str) -> Result<Option<Value>> {
        match self
            .store
            .latest_captcha_response(self.application_id, area, phone_no)?
        {
            Some(text) => Ok(Some(
                serde_json::from_str(&text)
                    .map_err(|e| crate::error::SdkError::Storage(e.to_string()))?,
            )),
            None => Ok(None),
        }
    }
}

pub struct ContactsApi<'a> {
    client: &'a mut ColaClient,
}

impl<'a> ContactsApi<'a> {
    /// # [HANDLER] - ContactsApi::new
    /// * `DESC`: `创建绑定 SDK 客户端的联系人 UI API`
    pub fn new(client: &'a mut ColaClient) -> Self {
        Self { client }
    }

    /// # [HANDLER] - ContactsApi::sync_incremental
    /// * `DESC`: `同步联系人增量批次并返回已提交结果`
    pub fn sync_incremental(&mut self, batch: &ContactDeltaBatch) -> Result<ContactSyncResult> {
        self.client.sync_contacts(batch)
    }

    /// # [HANDLER] - ContactsApi::list
    /// * `DESC`: `读取当前账号已提交的联系人列表`
    pub fn list(&self) -> Vec<ContactRecord> {
        self.client.contacts()
    }

    /// # [HANDLER] - ContactsApi::events
    /// * `DESC`: `获取并清空联系人及其他 UI 事件`
    pub fn events(&mut self) -> Vec<ClientEvent> {
        self.client.drain_events()
    }
}

pub struct MessagesApi<'a> {
    client: &'a mut ColaClient,
}

impl<'a> MessagesApi<'a> {
    /// # [HANDLER] - MessagesApi::new
    /// * `DESC`: `创建消息 UI API 门面`
    pub fn new(client: &'a mut ColaClient) -> Self {
        Self { client }
    }

    /// # [HANDLER] - MessagesApi::sync_offline
    /// * `DESC`: `从已解码的离线批次同步消息并返回提交结果`
    pub fn sync_offline(&mut self, batch: &OfflineMessageBatch) -> Result<SyncApplyResult> {
        self.client.sync_offline_messages(batch)
    }

    /// # [HANDLER] - MessagesApi::delete_local
    /// * `DESC`: `删除本地消息记录`
    pub fn delete_local(&mut self, conversation_id: &str, message_id: &str) -> Result<()> {
        self.client
            .delete_local_message(conversation_id, message_id)
    }

    /// # [HANDLER] - MessagesApi::events
    /// * `DESC`: `获取并清空 UI 事件队列`
    pub fn events(&mut self) -> Vec<ClientEvent> {
        self.client.drain_events()
    }
}

////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::domain::{AccountContext, MessageRecord};

    fn batch(account_id: &str, cursor: Option<&str>, next: &str) -> OfflineMessageBatch {
        OfflineMessageBatch {
            application_id: "cola_im".to_string(),
            account_id: account_id.to_string(),
            biz_id: "message".to_string(),
            previous_cursor: cursor.map(str::to_string),
            next_cursor: Some(next.to_string()),
            has_more: false,
            messages: vec![MessageRecord {
                message_id: "message-1".to_string(),
                conversation_id: "conversation-1".to_string(),
                server_version: 1,
                ciphertext: vec![1, 2, 3],
                deleted: false,
            }],
        }
    }

    fn contact_batch(account_id: &str, cursor: Option<&str>) -> ContactDeltaBatch {
        ContactDeltaBatch {
            application_id: "cola_im".to_string(),
            account_id: account_id.to_string(),
            biz_id: "contact".to_string(),
            previous_cursor: cursor.map(str::to_string),
            next_cursor: Some("contact-cursor-1".to_string()),
            has_more: false,
            contacts: vec![ContactRecord {
                contact_id: "account-b".to_string(),
                nickname: Some("Alice".to_string()),
                avatar_url: None,
                remark: Some("同事".to_string()),
                relationship: "friend".to_string(),
                presence: "online".to_string(),
                server_version: 1,
                updated_at: 100,
                deleted: false,
            }],
        }
    }

    #[test]
    fn offline_sync_commits_before_ui_events() {
        let context = AccountContext::new("cola_im", "account-a");
        let mut client = ColaClient::new(context);
        let result = client.sync_offline_messages(&batch("account-a", None, "cursor-1"));

        assert_eq!(result.unwrap().applied_count, 1);
        assert_eq!(client.messages().len(), 1);
        assert!(matches!(
            client.drain_events()[1],
            ClientEvent::OfflineMessagesSynced(_)
        ));
    }

    #[test]
    fn account_context_isolation_rejects_other_account_batch() {
        let context = AccountContext::new("cola_im", "account-a");
        let mut client = ColaClient::new(context);
        let result = client.sync_offline_messages(&batch("account-b", None, "cursor-1"));

        assert!(result.is_err());
        assert!(client.messages().is_empty());
        assert!(
            client
                .drain_events()
                .iter()
                .any(|event| matches!(event, ClientEvent::SyncFailed(_)))
        );
    }

    #[test]
    fn delete_emits_event_after_local_delete() {
        let context = AccountContext::new("cola_im", "account-a");
        let mut client = ColaClient::new(context);
        client
            .sync_offline_messages(&batch("account-a", None, "cursor-1"))
            .unwrap();
        client.drain_events();

        client
            .delete_local_message("conversation-1", "message-1")
            .unwrap();
        assert!(client.messages().is_empty());
        assert_eq!(
            client.drain_events(),
            vec![ClientEvent::LocalMessageDeleted {
                conversation_id: "conversation-1".to_string(),
                message_id: "message-1".to_string(),
            }]
        );
    }

    #[test]
    fn contacts_api_syncs_and_lists_current_account_contacts() {
        let context = AccountContext::new("cola_im", "account-a");
        let mut client = ColaClient::new(context);
        let mut api = ContactsApi::new(&mut client);

        let result = api
            .sync_incremental(&contact_batch("account-a", None))
            .unwrap();

        assert_eq!(result.applied_count, 1);
        assert_eq!(result.next_cursor.as_deref(), Some("contact-cursor-1"));
        assert_eq!(api.list()[0].contact_id, "account-b");
        assert!(
            api.events()
                .iter()
                .any(|event| matches!(event, ClientEvent::ContactSyncCompleted(_)))
        );
    }

    #[test]
    fn contacts_api_rejects_wrong_account_without_writing() {
        let context = AccountContext::new("cola_im", "account-a");
        let mut client = ColaClient::new(context);
        let mut api = ContactsApi::new(&mut client);

        assert!(
            api.sync_incremental(&contact_batch("account-b", None))
                .is_err()
        );
        assert!(api.list().is_empty());
        assert!(
            api.events()
                .iter()
                .any(|event| matches!(event, ClientEvent::SyncFailed(_)))
        );
    }
}

//////// END
