// src/config/mod.rs -- Embedded non-sensitive application defaults
// 2026/10/11 01:10 Created.

////////

/// 编译期嵌入的非敏感默认配置。
///
/// 运行时配置应由 Android/iOS 宿主通过平台安全接口注入并优先使用。
pub const EMBEDDED_APPLICATION_TOML: &str = include_str!("../../application.toml");

/// # [HANDLER] - embedded_application_toml
/// * `DESC`: `返回编译期嵌入的默认应用配置文本`
pub const fn embedded_application_toml() -> &'static str {
    EMBEDDED_APPLICATION_TOML
}

#[cfg(test)]
mod tests {
    use super::EMBEDDED_APPLICATION_TOML;

    #[test]
    fn embeds_non_sensitive_application_defaults() {
        assert!(EMBEDDED_APPLICATION_TOML.contains("Cola IM Client SDK"));
        assert!(EMBEDDED_APPLICATION_TOML.contains("Global-Gateway.1"));
        assert!(!EMBEDDED_APPLICATION_TOML.contains("api_key"));
        assert!(!EMBEDDED_APPLICATION_TOML.contains("token"));
    }
}

//////// END
