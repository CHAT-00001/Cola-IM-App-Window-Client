// cola_data/src/api/cola_data.rs -- DATA - APP - data  （统一应用数据响应壳）
// 2026/5/22 13:46

////////

// 移除了被硬编码的 comment_list_response 引入
use std::time::Instant;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # [DATA] - 统一应用数据体
/// * `desc`: `数据响应壳`
#[derive(Serialize, Debug)]
pub struct AppData<T> {
    pub code: i32,          // 0: 成功, 其他: 错误码
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub duration: String,
    pub request_id: String,
    pub at: String,
    pub log_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}


/// # [BUILD] - 构造响应壳
impl<T> AppData<T> {
    ////////

    /// # [CASE] - ✅ 成功响应（恢复彻底泛型化）
    /// `desc` `现在它可以接收 String, AuthSessionResponse, CommentListResponse 等任何类型`
    pub fn ok(data: T) -> Self { // 👈 核心修改：改为接收泛型 T
        Self {
            code: 0,
            message: "success".into(),
            error: None,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: Some(data),
        }
    }

    ////////

    /// # [CASE] - 🌐 成功响应（无数据返回）
    /// `desc`: `改进：直接返回 AppData<T> 以适配上下文，不需要硬写死为 AppData<()>`
    pub fn empty() -> Self {
        Self {
            code: 0,
            message: "success".into(),
            error: None,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: None,
        }
    }

    ////////

    /// # [CASE] - ❌️ 错误响应
    /// * `desc`: `错误信息`
    pub fn err(code: i32, message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            code,
            message: message.into(),
            error: detail,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: None,
        }
    }

    ////////

    /// # [CASE] - 🔗 链式调用：修改消息内容
    /// * `desc`: `可以注入错误信息`
    pub fn with_msg(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    ////////

    /// # [CASE] - 🍚 泛型重绑定
    /// * `desc`: `支持从 AppData<A> 转换到 AppData<B>`
    pub fn rebind<U>(self) -> AppData<U> {
        AppData {
            code: self.code,
            message: self.message,
            error: self.error,
            duration: self.duration,
            request_id: self.request_id,
            at: self.at,
            log_id: self.log_id,
            data: None,
        }
    }

    ////////

    /// # [CASE] - 🔍 检查响应是否成功
    pub fn check(self) -> Result<T, AppData<T>> {
        if self.code == 0 {
            Ok(self.data.expect("Data missing"))
        } else {
            Err(self)
        }
    }
}

//////// END