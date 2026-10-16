// src/domain/mod.rs -- Cola IM synchronization domain models
// 2026/10/9 00:20 Created.

////////

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountContext {
    pub application_id: String,
    pub account_id: String,
}

impl AccountContext {
    /// # [HANDLER] - AccountContext::new
    /// * `DESC`: `创建绑定应用和当前登录账号的数据上下文`
    pub fn new(application_id: impl Into<String>, account_id: impl Into<String>) -> Self {
        Self {
            application_id: application_id.into(),
            account_id: account_id.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageRecord {
    pub message_id: String,
    pub conversation_id: String,
    pub server_version: i64,
    pub ciphertext: Vec<u8>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OfflineMessageBatch {
    pub application_id: String,
    pub account_id: String,
    pub biz_id: String,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub messages: Vec<MessageRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SyncApplyResult {
    pub applied_count: u32,
    pub deleted_count: u32,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactRecord {
    pub contact_id: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub remark: Option<String>,
    pub relationship: String,
    pub presence: String,
    pub server_version: i64,
    pub updated_at: i64,
    pub deleted: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactDeltaBatch {
    pub application_id: String,
    pub account_id: String,
    pub biz_id: String,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub contacts: Vec<ContactRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactSyncResult {
    pub applied_count: u32,
    pub deleted_count: u32,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactDeleteRequest {
    pub conversation_id: String,
    pub message_id: String,
}

//////// END
