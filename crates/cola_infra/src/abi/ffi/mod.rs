// cola_infra/src/abi/ffi/mod -- INFRA - C ABI接入层 - mod
// 2026/10/11 02:20 Created.

////////

use crate::client::{ClientEvent, ColaClient};
use crate::network::domain::{
    AccountContext, ContactDeltaBatch, ContactRecord, ContactSyncResult,
};
use crate::network::endpoint::EndpointRegistry;
use crate::network::http::HttpSender;
use crate::storage::sqlite::SqliteStore;
use crate::ui_api::{AuthApi, ContactsApi};
use std::ffi::{CStr, CString, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;

////////

// 所有导出函数均返回稳定的整数错误码，不跨 ABI 边界返回 Rust Result。
pub const COLA_OK: i32 = 0;
pub const COLA_ERR_INVALID_ARGUMENT: i32 = 1;
pub const COLA_ERR_INVALID_CONTEXT: i32 = 2;
pub const COLA_ERR_INVALID_BATCH: i32 = 3;
pub const COLA_ERR_CURSOR_MISMATCH: i32 = 4;
pub const COLA_ERR_NOT_FOUND: i32 = 5;
pub const COLA_ERR_PANIC: i32 = 100;

// 事件码由 Swift/Kotlin/Java 映射为各自平台的事件枚举。
pub const COLA_EVENT_NONE: i32 = 0;
pub const COLA_EVENT_CONTACT_SYNC_STARTED: i32 = 10;
pub const COLA_EVENT_CONTACTS_CHANGED: i32 = 11;
pub const COLA_EVENT_CONTACT_SYNC_COMPLETED: i32 = 12;
pub const COLA_EVENT_SYNC_FAILED: i32 = 13;

#[repr(C)]
pub struct ColaClientHandle {
    // 句柄由 cola_client_create 创建，只能通过 cola_client_destroy 释放。
    client: ColaClient,
    last_error: Option<CString>,
}

#[repr(C)]
pub struct ColaContactInput {
    // 输入字符串必须是以 NUL 结尾的 UTF-8 C 字符串；可选字段允许为空指针。
    pub contact_id: *const c_char,
    pub nickname: *const c_char,
    pub avatar_url: *const c_char,
    pub remark: *const c_char,
    pub relationship: *const c_char,
    pub presence: *const c_char,
    pub server_version: i64,
    pub updated_at: i64,
    pub deleted: bool,
}

#[repr(C)]
pub struct ColaContactBatchInput {
    // contacts 指向调用方持有的数组，仅在同步函数调用期间读取，不会由 Rust 释放。
    pub application_id: *const c_char,
    pub account_id: *const c_char,
    pub biz_id: *const c_char,
    pub previous_cursor: *const c_char,
    pub next_cursor: *const c_char,
    pub has_more: bool,
    pub contacts: *const ColaContactInput,
    pub contacts_len: usize,
}

#[repr(C)]
pub struct ColaContactOutput {
    // 输出字符串由 Rust 分配，调用方必须使用 cola_contact_list_free 统一释放。
    pub contact_id: *mut c_char,
    pub nickname: *mut c_char,
    pub avatar_url: *mut c_char,
    pub remark: *mut c_char,
    pub relationship: *mut c_char,
    pub presence: *mut c_char,
    pub server_version: i64,
    pub updated_at: i64,
}

#[repr(C)]
pub struct ColaContactList {
    // items/len 组成连续数组，不能使用 free() 释放。
    pub items: *mut ColaContactOutput,
    pub len: usize,
}

#[repr(C)]
pub struct ColaContactSyncResult {
    // next_cursor 由 Rust 分配，使用 cola_contact_sync_result_free 释放。
    pub applied_count: u32,
    pub deleted_count: u32,
    pub next_cursor: *mut c_char,
    pub has_more: bool,
}

#[repr(C)]
pub struct ColaEvent {
    // 事件结构只携带轻量摘要；详细联系人数据通过 list_contacts 读取。
    pub kind: i32,
    pub added_or_updated_count: u32,
    pub deleted_count: u32,
}

fn required(value: *const c_char) -> Result<String, i32> {
    if value.is_null() {
        return Err(COLA_ERR_INVALID_ARGUMENT);
    }
    unsafe { CStr::from_ptr(value) }
        .to_str()
        .map(str::to_owned)
        .map_err(|_| COLA_ERR_INVALID_ARGUMENT)
}

fn optional(value: *const c_char) -> Result<Option<String>, i32> {
    if value.is_null() {
        Ok(None)
    } else {
        required(value).map(Some)
    }
}

fn into_c_string(value: Option<String>) -> *mut c_char {
    value
        .and_then(|value| CString::new(value).ok())
        .map_or(ptr::null_mut(), CString::into_raw)
}

fn set_error(handle: &mut ColaClientHandle, code: i32, message: impl Into<String>) -> i32 {
    handle.last_error = CString::new(message.into()).ok();
    code
}

fn map_error(handle: &mut ColaClientHandle, error: impl std::fmt::Debug) -> i32 {
    let message = format!("{error:?}");
    let code = if message.contains("InvalidContext") {
        COLA_ERR_INVALID_CONTEXT
    } else if message.contains("InvalidBatch") {
        COLA_ERR_INVALID_BATCH
    } else if message.contains("CursorMismatch") {
        COLA_ERR_CURSOR_MISMATCH
    } else if message.contains("NotFound") {
        COLA_ERR_NOT_FOUND
    } else {
        COLA_ERR_INVALID_ARGUMENT
    };
    set_error(handle, code, message)
}

#[unsafe(no_mangle)]
/// 创建一个绑定 application_id 和 account_id 的不透明 SDK 句柄。
pub extern "C" fn cola_client_create(
    application_id: *const c_char,
    account_id: *const c_char,
    out_handle: *mut *mut ColaClientHandle,
) -> i32 {
    if out_handle.is_null() {
        return COLA_ERR_INVALID_ARGUMENT;
    }
    match catch_unwind(AssertUnwindSafe(|| {
        let application_id = required(application_id)?;
        let account_id = required(account_id)?;
        let handle = Box::new(ColaClientHandle {
            client: ColaClient::new(AccountContext::new(application_id, account_id)),
            last_error: None,
        });
        unsafe { *out_handle = Box::into_raw(handle) };
        Ok::<(), i32>(())
    })) {
        Ok(Ok(())) => COLA_OK,
        Ok(Err(code)) => code,
        Err(_) => COLA_ERR_PANIC,
    }
}

#[unsafe(no_mangle)]
/// 销毁句柄。传入空指针安全无操作；销毁后不得继续使用该句柄。
pub unsafe extern "C" fn cola_client_destroy(handle: *mut ColaClientHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)) };
    }
}

#[unsafe(no_mangle)]
/// 将已由平台层解码的联系人增量批次提交到 SDK。
/// 成功后结果中的游标由调用方负责释放，事件可通过 cola_client_poll_event 获取。
pub unsafe extern "C" fn cola_client_sync_contacts(
    handle: *mut ColaClientHandle,
    input: *const ColaContactBatchInput,
    output: *mut ColaContactSyncResult,
) -> i32 {
    if handle.is_null() || input.is_null() || output.is_null() {
        return COLA_ERR_INVALID_ARGUMENT;
    }
    match catch_unwind(AssertUnwindSafe(|| {
        let handle = unsafe { &mut *handle };
        let input = unsafe { &*input };
        let batch = match unsafe { build_batch(input) } {
            Ok(batch) => batch,
            Err(code) => return set_error(handle, code, "invalid contact batch"),
        };
        match ContactsApi::new(&mut handle.client).sync_incremental(&batch) {
            Ok(result) => {
                unsafe { *output = to_c_result(result) };
                COLA_OK
            }
            Err(error) => map_error(handle, error),
        }
    })) {
        Ok(code) => code,
        Err(_) => COLA_ERR_PANIC,
    }
}

unsafe fn build_batch(input: &ColaContactBatchInput) -> Result<ContactDeltaBatch, i32> {
    if input.contacts_len > 0 && input.contacts.is_null() {
        return Err(COLA_ERR_INVALID_ARGUMENT);
    }
    let contacts = if input.contacts_len == 0 {
        Vec::new()
    } else {
        let values = unsafe { std::slice::from_raw_parts(input.contacts, input.contacts_len) };
        values
            .iter()
            .map(build_contact)
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok(ContactDeltaBatch {
        application_id: required(input.application_id)?,
        account_id: required(input.account_id)?,
        biz_id: required(input.biz_id)?,
        previous_cursor: optional(input.previous_cursor)?,
        next_cursor: optional(input.next_cursor)?,
        has_more: input.has_more,
        contacts,
    })
}

////////

/// # [BUILD] - 构造联系人
fn build_contact(input: &ColaContactInput) -> Result<ContactRecord, i32> {
    Ok(ContactRecord {
        contact_id: required(input.contact_id)?,
        nickname: optional(input.nickname)?,
        avatar_url: optional(input.avatar_url)?,
        remark: optional(input.remark)?,
        relationship: required(input.relationship)?,
        presence: required(input.presence)?,
        server_version: input.server_version,
        updated_at: input.updated_at,
        deleted: input.deleted,
    })
}

fn to_c_result(result: ContactSyncResult) -> ColaContactSyncResult {
    ColaContactSyncResult {
        applied_count: result.applied_count,
        deleted_count: result.deleted_count,
        next_cursor: into_c_string(result.next_cursor),
        has_more: result.has_more,
    }
}

#[unsafe(no_mangle)]
/// 导出当前 account_id 已提交的联系人快照。
pub unsafe extern "C" fn cola_client_list_contacts(
    handle: *mut ColaClientHandle,
    output: *mut ColaContactList,
) -> i32 {
    if handle.is_null() || output.is_null() {
        return COLA_ERR_INVALID_ARGUMENT;
    }
    match catch_unwind(AssertUnwindSafe(|| {
        let handle = unsafe { &mut *handle };
        let mut values: Vec<ColaContactOutput> = handle
            .client
            .contacts()
            .into_iter()
            .map(|contact| ColaContactOutput {
                contact_id: into_c_string(Some(contact.contact_id)),
                nickname: into_c_string(contact.nickname),
                avatar_url: into_c_string(contact.avatar_url),
                remark: into_c_string(contact.remark),
                relationship: into_c_string(Some(contact.relationship)),
                presence: into_c_string(Some(contact.presence)),
                server_version: contact.server_version,
                updated_at: contact.updated_at,
            })
            .collect();
        unsafe {
            *output = ColaContactList {
                items: values.as_mut_ptr(),
                len: values.len(),
            };
        }
        std::mem::forget(values);
        COLA_OK
    })) {
        Ok(code) => code,
        Err(_) => COLA_ERR_PANIC,
    }
}

#[unsafe(no_mangle)]
/// 一次取出一个事件，不会丢弃队列中的后续事件。
pub unsafe extern "C" fn cola_client_poll_event(
    handle: *mut ColaClientHandle,
    output: *mut ColaEvent,
) -> i32 {
    if handle.is_null() || output.is_null() {
        return COLA_ERR_INVALID_ARGUMENT;
    }
    let handle = unsafe { &mut *handle };
    let event = handle.client.pop_event();
    let value = match event {
        Some(ClientEvent::ContactSyncStarted) => ColaEvent {
            kind: COLA_EVENT_CONTACT_SYNC_STARTED,
            added_or_updated_count: 0,
            deleted_count: 0,
        },
        Some(ClientEvent::ContactsChanged {
            added_or_updated,
            deleted,
        }) => ColaEvent {
            kind: COLA_EVENT_CONTACTS_CHANGED,
            added_or_updated_count: added_or_updated.len() as u32,
            deleted_count: deleted.len() as u32,
        },
        Some(ClientEvent::ContactSyncCompleted(result)) => ColaEvent {
            kind: COLA_EVENT_CONTACT_SYNC_COMPLETED,
            added_or_updated_count: result.applied_count.saturating_sub(result.deleted_count),
            deleted_count: result.deleted_count,
        },
        Some(ClientEvent::SyncFailed(_)) => ColaEvent {
            kind: COLA_EVENT_SYNC_FAILED,
            added_or_updated_count: 0,
            deleted_count: 0,
        },
        _ => ColaEvent {
            kind: COLA_EVENT_NONE,
            added_or_updated_count: 0,
            deleted_count: 0,
        },
    };
    unsafe { *output = value };
    COLA_OK
}

#[unsafe(no_mangle)]
/// 释放联系人同步结果中的 next_cursor。
pub unsafe extern "C" fn cola_contact_sync_result_free(result: *mut ColaContactSyncResult) {
    if !result.is_null() {
        unsafe { free_string(&mut (*result).next_cursor) };
    }
}

#[unsafe(no_mangle)]
/// 释放联系人数组及其中的所有 Rust 分配字符串。
pub unsafe extern "C" fn cola_contact_list_free(list: *mut ColaContactList) {
    if list.is_null() {
        return;
    }
    let list = unsafe { &mut *list };
    if !list.items.is_null() {
        let items = unsafe { std::slice::from_raw_parts_mut(list.items, list.len) };
        for item in items {
            unsafe {
                free_string(&mut item.contact_id);
                free_string(&mut item.nickname);
                free_string(&mut item.avatar_url);
                free_string(&mut item.remark);
                free_string(&mut item.relationship);
                free_string(&mut item.presence);
            }
        }
        let slice = ptr::slice_from_raw_parts_mut(list.items, list.len);
        unsafe { drop(Box::from_raw(slice)) };
    }
    list.items = ptr::null_mut();
    list.len = 0;
}

#[unsafe(no_mangle)]
/// 获取最近一次错误的 UTF-8 文本；指针归句柄所有，不需要调用者释放。
pub unsafe extern "C" fn cola_client_last_error(handle: *const ColaClientHandle) -> *const c_char {
    if handle.is_null() {
        return ptr::null();
    }
    unsafe {
        (*handle)
            .last_error
            .as_ref()
            .map_or(ptr::null(), |error| error.as_ptr())
    }
}

#[unsafe(no_mangle)]
/// 释放由 Rust 返回的独立 C 字符串。
pub unsafe extern "C" fn cola_string_free(value: *mut c_char) {
    if !value.is_null() {
        unsafe { drop(CString::from_raw(value)) };
    }
}

#[unsafe(no_mangle)]
/// 发送验证码: 成功返回服务端 JSON 文本 (UTF-8, 调用方用 cola_string_free 释放); 失败返回 NULL。
/// `db_path` 为本地 SQLite 文件路径, 验证码记录按 application_id 隔离落库。
pub unsafe extern "C" fn cola_auth_send_captcha(
    application_id: *const c_char,
    db_path: *const c_char,
    area: *const c_char,
    phone_no: *const c_char,
) -> *mut c_char {
    let (Ok(application_id), Ok(db_path), Ok(area), Ok(phone_no)) = (
        required(application_id),
        required(db_path),
        required(area),
        required(phone_no),
    ) else {
        return ptr::null_mut();
    };
    let result = catch_unwind(AssertUnwindSafe(|| {
        let store = SqliteStore::open(std::path::Path::new(&db_path)).ok()?;
        let registry = EndpointRegistry::from_builtin();
        let sender = HttpSender::new(std::time::Duration::from_secs(10));
        AuthApi::new(&registry, &sender, &store, &application_id)
            .send_captcha(&area, &phone_no)
            .ok()
            .map(|json| json.to_string())
    }));
    match result {
        Ok(text) => into_c_string(text),
        Err(_) => ptr::null_mut(),
    }
}

unsafe fn free_string(value: &mut *mut c_char) {
    if !(*value).is_null() {
        unsafe { drop(CString::from_raw(*value)) };
        *value = ptr::null_mut();
    }
}
