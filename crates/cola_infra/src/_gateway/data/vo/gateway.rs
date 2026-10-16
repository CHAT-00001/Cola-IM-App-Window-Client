// src/gateway/data/vo/gateway.rs -- DATA - vo - 网关视图
// 2026/10/11 06:16 Created.

////////

/// # [VO] - 网关
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayCandidate {
    pub id: String,
    pub domain: String,
    pub port: u16,
    pub refresh_path: String,
}
