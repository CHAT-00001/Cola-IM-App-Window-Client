// src/endpoint/probe.rs -- Endpoint probing interfaces and test implementations
// 2026/10/9 00:25 Created.

////////

use super::model::{EndpointAddress, EndpointProtocol, ProbeResult, TransportKind};

/// # [HANDLER] - EndpointProbe::probe
/// * `DESC`: `使用指定传输方式测试接入点并返回状态结果`
pub trait EndpointProbe {
    fn probe(
        &self,
        endpoint_id: &str,
        address: &EndpointAddress,
        protocols: EndpointProtocol,
        transport: TransportKind,
    ) -> ProbeResult;
}

#[derive(Debug, Clone, Copy)]
pub struct StaticProbe {
    pub result: bool,
    pub latency_ms: Option<u64>,
    pub checked_at_ms: u64,
}

impl StaticProbe {
    /// # [HANDLER] - StaticProbe::new
    /// * `DESC`: `创建用于单元测试和离线验证的固定探测器`
    pub const fn new(result: bool, latency_ms: Option<u64>, checked_at_ms: u64) -> Self {
        Self {
            result,
            latency_ms,
            checked_at_ms,
        }
    }
}

impl EndpointProbe for StaticProbe {
    fn probe(
        &self,
        endpoint_id: &str,
        _address: &EndpointAddress,
        _protocols: EndpointProtocol,
        transport: TransportKind,
    ) -> ProbeResult {
        ProbeResult {
            endpoint_id: endpoint_id.to_string(),
            transport,
            ok: self.result,
            latency_ms: self.latency_ms,
            checked_at_ms: self.checked_at_ms,
            message: (!self.result).then(|| "static probe failed".to_string()),
        }
    }
}

//////// END
