// cola_data/src/app/context.rs -- DATA - APP - 请求上下文
// 2026/10/13 Created.

////////

/// # 1. [DATA APP] - 请求上下文
/// * `DESC`: 保存与业务实现无关的请求元数据
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    pub request_id: Option<String>, // 请求 ID
    pub uid: Option<i64>,            // 用户 ID
    pub lang: Option<String>,        // 语言
}

//////// END
