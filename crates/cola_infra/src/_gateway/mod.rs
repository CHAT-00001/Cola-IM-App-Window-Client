// src/gateway/mod.rs -- Global Gateway probing and endpoint refresh
// 2026/10/11 00:45 Created.

////////

mod data;

use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use crate::endpoint::{EndpointRegistry, GatewayEndpoint};
use crate::error::{Result, SdkError};

////////

/// # [QUANTY] - 网关数量
const MAX_GATEWAYS: usize = 4;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayCandidate {
    pub id: String,
    pub domain: String,
    pub port: u16,
    pub refresh_path: String,
}

impl GatewayCandidate {
    pub fn new(
        id: impl Into<String>,
        domain: impl Into<String>,
        port: u16,
        refresh_path: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            domain: domain.into(),
            port,
            refresh_path: refresh_path.into(),
        }
    }

    fn address(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayProbeResult {
    pub gateway_id: String,
    pub reachable: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct GatewayStatus {
    pub last_probe: Option<GatewayProbeResult>,
    pub consecutive_failures: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewaySnapshot {
    pub config_version: u64,
    pub endpoints: Vec<GatewayEndpoint>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayRefreshResult {
    pub gateway_id: String,
    pub latency_ms: u64,
    pub config_version: u64,
    pub endpoint_count: usize,
}

pub trait GatewayProbe {
    fn probe(&self, gateway: &GatewayCandidate, timeout: Duration) -> GatewayProbeResult;
}

pub trait GatewaySnapshotFetcher {
    fn fetch(&self, gateway: &GatewayCandidate, timeout: Duration) -> Result<GatewaySnapshot>;
}

// TCP 握手实现
#[derive(Debug, Clone, Copy)]
pub struct TcpGatewayProbe;

impl GatewayProbe for TcpGatewayProbe {
    fn probe(&self, gateway: &GatewayCandidate, timeout: Duration) -> GatewayProbeResult {
        let started = Instant::now();
        let result = gateway
            .address()
            .to_socket_addrs()
            .map_err(|error| error.to_string())
            .and_then(|addresses| {
                let mut last_error = None;
                for address in addresses {
                    match TcpStream::connect_timeout(&address, timeout) {
                        Ok(stream) => {
                            drop(stream);
                            return Ok(());
                        }
                        Err(error) => last_error = Some(error.to_string()),
                    }
                }
                Err(last_error.unwrap_or_else(|| "no address resolved".to_string()))
            });

        match result {
            Ok(()) => GatewayProbeResult {
                gateway_id: gateway.id.clone(),
                reachable: true,
                latency_ms: Some(started.elapsed().as_millis() as u64),
                error: None,
            },
            Err(error) => GatewayProbeResult {
                gateway_id: gateway.id.clone(),
                reachable: false,
                latency_ms: None,
                error: Some(error),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct GatewayManager<P = TcpGatewayProbe> {
    candidates: Vec<GatewayCandidate>,
    statuses: Vec<GatewayStatus>,
    probe: P,
    timeout: Duration,
    selected_gateway: Option<usize>,
}

impl GatewayManager<TcpGatewayProbe> {
    /// # [HANDLER] - GatewayManager::from_builtin
    /// * `DESC`: `创建最多四个候选 Global Gateway 的管理器`
    pub fn from_builtin() -> Self {
        Self::new(
            vec![GatewayCandidate::new(
                "global-gateway-primary",
                "global-gateway.cola.example.com",
                443,
                "/v1/client/endpoints",
            )],
            TcpGatewayProbe,
            Duration::from_secs(3),
        )
        .expect("builtin gateway list must be valid")
    }
}

impl<P: GatewayProbe> GatewayManager<P> {
    /// # [HANDLER] - GatewayManager::new
    /// * `DESC`: `创建并校验一到四个 Global Gateway 候选`
    pub fn new(candidates: Vec<GatewayCandidate>, probe: P, timeout: Duration) -> Result<Self> {
        if candidates.is_empty() || candidates.len() > MAX_GATEWAYS {
            return Err(SdkError::InvalidContext(
                "global gateway count must be between 1 and 4".to_string(),
            ));
        }
        if candidates
            .iter()
            .any(|gateway| gateway.domain.is_empty() || gateway.port == 0)
        {
            return Err(SdkError::InvalidContext(
                "global gateway domain and port are required".to_string(),
            ));
        }
        Ok(Self {
            statuses: vec![GatewayStatus::default(); candidates.len()],
            candidates,
            probe,
            timeout,
            selected_gateway: None,
        })
    }

    pub fn candidates(&self) -> &[GatewayCandidate] {
        &self.candidates
    }

    pub fn statuses(&self) -> &[GatewayStatus] {
        &self.statuses
    }

    pub fn selected_gateway(&self) -> Option<&GatewayCandidate> {
        self.selected_gateway.map(|index| &self.candidates[index])
    }

    /// # [HANDLER] - GatewayManager::probe_all
    /// * `DESC`: `探测全部候选网关并按可达性和延迟选择网关`
    pub fn probe_all(&mut self) -> Vec<GatewayProbeResult> {
        let mut results = Vec::with_capacity(self.candidates.len());
        let mut best: Option<(usize, u64)> = None;
        for (index, gateway) in self.candidates.iter().enumerate() {
            let result = self.probe.probe(gateway, self.timeout);
            let status = &mut self.statuses[index];
            status.last_probe = Some(result.clone());
            status.consecutive_failures = if result.reachable {
                0
            } else {
                status.consecutive_failures.saturating_add(1)
            };
            if let (true, Some(latency)) = (result.reachable, result.latency_ms) {
                if best.is_none_or(|(_, best_latency)| latency < best_latency) {
                    best = Some((index, latency));
                }
            }
            results.push(result);
        }
        self.selected_gateway = best.map(|(index, _)| index);
        results
    }

    /// # [HANDLER] - GatewayManager::refresh_endpoints
    /// * `DESC`: `从首选可达 Global Gateway 获取并替换本地 Endpoint 快照`
    pub fn refresh_endpoints<F>(
        &mut self,
        registry: &mut EndpointRegistry,
        fetcher: &F,
    ) -> Result<GatewayRefreshResult>
    where
        F: GatewaySnapshotFetcher,
    {
        self.probe_all();
        let index = self
            .selected_gateway
            .ok_or_else(|| SdkError::Storage("no reachable global gateway".to_string()))?;
        let gateway = &self.candidates[index];
        let started = Instant::now();
        let snapshot = fetcher.fetch(gateway, self.timeout)?;
        let endpoint_count = snapshot.endpoints.len();
        registry.replace_endpoints(snapshot.endpoints);
        Ok(GatewayRefreshResult {
            gateway_id: gateway.id.clone(),
            latency_ms: started.elapsed().as_millis() as u64,
            config_version: snapshot.config_version,
            endpoint_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint::{EndpointProtocol, EndpointStatus, TransportKind};

    #[derive(Debug, Clone)]
    struct FakeProbe(Vec<GatewayProbeResult>);

    impl GatewayProbe for FakeProbe {
        fn probe(&self, gateway: &GatewayCandidate, _timeout: Duration) -> GatewayProbeResult {
            self.0
                .iter()
                .find(|result| result.gateway_id == gateway.id)
                .cloned()
                .unwrap()
        }
    }

    struct FakeFetcher;

    impl GatewaySnapshotFetcher for FakeFetcher {
        fn fetch(&self, gateway: &GatewayCandidate, _timeout: Duration) -> Result<GatewaySnapshot> {
            assert_eq!(gateway.id, "gateway-2");
            Ok(GatewaySnapshot {
                config_version: 7,
                endpoints: vec![GatewayEndpoint {
                    id: "contact-edge-1".to_string(),
                    biz_id: "contact".to_string(),
                    priority: 100,
                    status: EndpointStatus::Active,
                    address: crate::endpoint::EndpointAddress::ip_with_sni(
                        "203.0.113.20",
                        443,
                        "contact-edge.cola.example.com",
                    ),
                    protocols: EndpointProtocol::http_websocket(),
                    health: Default::default(),
                }],
            })
        }
    }

    fn candidates() -> Vec<GatewayCandidate> {
        vec![
            GatewayCandidate::new("gateway-1", "gateway-1.example", 443, "/v1/endpoints"),
            GatewayCandidate::new("gateway-2", "gateway-2.example", 443, "/v1/endpoints"),
        ]
    }

    #[test]
    fn rejects_more_than_four_gateways() {
        let candidates = (0..5)
            .map(|index| {
                GatewayCandidate::new(
                    format!("gateway-{index}"),
                    format!("gateway-{index}.example"),
                    443,
                    "/v1/endpoints",
                )
            })
            .collect();
        assert!(
            GatewayManager::new(candidates, FakeProbe(vec![]), Duration::from_millis(1)).is_err()
        );
    }

    #[test]
    fn selects_fastest_reachable_gateway() {
        let probe = FakeProbe(vec![
            GatewayProbeResult {
                gateway_id: "gateway-1".to_string(),
                reachable: true,
                latency_ms: Some(80),
                error: None,
            },
            GatewayProbeResult {
                gateway_id: "gateway-2".to_string(),
                reachable: true,
                latency_ms: Some(20),
                error: None,
            },
        ]);
        let mut manager =
            GatewayManager::new(candidates(), probe, Duration::from_millis(10)).unwrap();
        manager.probe_all();
        assert_eq!(manager.selected_gateway().unwrap().id, "gateway-2");
    }

    #[test]
    fn refresh_replaces_registry_after_gateway_selection() {
        let probe = FakeProbe(vec![
            GatewayProbeResult {
                gateway_id: "gateway-1".to_string(),
                reachable: false,
                latency_ms: None,
                error: Some("timeout".to_string()),
            },
            GatewayProbeResult {
                gateway_id: "gateway-2".to_string(),
                reachable: true,
                latency_ms: Some(20),
                error: None,
            },
        ]);
        let mut manager =
            GatewayManager::new(candidates(), probe, Duration::from_millis(10)).unwrap();
        let mut registry = EndpointRegistry::from_builtin();
        let result = manager
            .refresh_endpoints(&mut registry, &FakeFetcher)
            .unwrap();

        assert_eq!(result.config_version, 7);
        assert_eq!(registry.endpoints()[0].biz_id, "contact");
        assert!(registry.select("contact", TransportKind::Http).is_ok());
    }
}

//////// END
