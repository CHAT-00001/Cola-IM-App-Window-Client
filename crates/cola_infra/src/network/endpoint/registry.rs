// src/endpoint/registry.rs -- Gateway and business endpoint registry
// 2026/10/9 00:25 Created.

////////

use super::model::{
    BuiltinGateway, BuiltinRoute, EndpointStatus, GatewayEndpoint, ProbeResult, RouteHealth,
    TransportKind,
};
use super::probe::EndpointProbe;

const BUILTIN_GATEWAYS: &[BuiltinGateway] = &[BuiltinGateway {
    id: "global-gateway-primary",
    domain: "global-gateway.cola.example.com",
    port: 443,
    refresh_path: "/v1/client/endpoints",
}];

fn builtin_routes() -> Vec<BuiltinRoute> {
    vec![
        BuiltinRoute {
            endpoint_id: "im-cn-east-1",
            biz_id: "im",
            priority: 100,
            status: EndpointStatus::Active,
            address: super::model::EndpointAddress::ip_with_sni(
                "203.0.113.10",
                443,
                "im-edge.cola.example.com",
            ),
            protocols: super::model::EndpointProtocol::http_websocket(),
        },
        BuiltinRoute {
            endpoint_id: "im-cn-backup-1",
            biz_id: "im",
            priority: 80,
            status: EndpointStatus::Active,
            address: super::model::EndpointAddress::ip_with_sni(
                "203.0.113.11",
                443,
                "im-edge.cola.example.com",
            ),
            protocols: super::model::EndpointProtocol::http_websocket(),
        },
        BuiltinRoute {
            endpoint_id: "auth-cn-east-1",
            biz_id: "auth",
            priority: 100,
            status: EndpointStatus::Active,
            address: super::model::EndpointAddress::ip_with_sni(
                "127.0.0.1",
                8080,
                "auth-edge.cola.example.com",
            ),
            protocols: super::model::EndpointProtocol::http_websocket(),
        },
        BuiltinRoute {
            endpoint_id: "upload-cn-east-1",
            biz_id: "file-upload",
            priority: 100,
            status: EndpointStatus::Active,
            address: super::model::EndpointAddress::ip_with_sni(
                "198.51.100.10",
                443,
                "upload-edge.cola.example.com",
            ),
            protocols: super::model::EndpointProtocol::http_websocket(),
        },
    ]
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegistryError {
    NoMatchingEndpoint {
        biz_id: String,
        transport: TransportKind,
    },
    UnknownEndpoint(String),
}

#[derive(Debug, Clone)]
pub struct EndpointRegistry {
    gateways: Vec<BuiltinGateway>,
    endpoints: Vec<GatewayEndpoint>,
}

impl EndpointRegistry {
    /// # [HANDLER] - EndpointRegistry::from_builtin
    /// * `DESC`: `加载内置全球网关和多业务兜底接入点表`
    pub fn from_builtin() -> Self {
        Self {
            gateways: BUILTIN_GATEWAYS.to_vec(),
            endpoints: builtin_routes()
                .into_iter()
                .map(GatewayEndpoint::from_builtin)
                .collect(),
        }
    }

    /// # [HANDLER] - EndpointRegistry::gateways
    /// * `DESC`: `返回当前注册表中的全球网关列表`
    pub fn gateways(&self) -> &[BuiltinGateway] {
        &self.gateways
    }

    /// # [HANDLER] - EndpointRegistry::endpoints
    /// * `DESC`: `返回当前所有业务接入点及其状态`
    pub fn endpoints(&self) -> &[GatewayEndpoint] {
        &self.endpoints
    }

    /// # [HANDLER] - EndpointRegistry::replace_endpoints
    /// * `DESC`: `用已验证的 Global Gateway 快照替换本地业务接入点`
    pub fn replace_endpoints(&mut self, endpoints: Vec<GatewayEndpoint>) {
        self.endpoints = endpoints;
    }

    /// # [HANDLER] - EndpointRegistry::for_biz
    /// * `DESC`: `按业务标识筛选接入点`
    pub fn for_biz(&self, biz_id: &str) -> impl Iterator<Item = &GatewayEndpoint> {
        self.endpoints
            .iter()
            .filter(move |endpoint| endpoint.biz_id == biz_id)
    }

    /// # [HANDLER] - EndpointRegistry::select
    /// * `DESC`: `按业务、协议、状态和优先级选择当前最佳接入点`
    pub fn select(
        &self,
        biz_id: &str,
        transport: TransportKind,
    ) -> Result<&GatewayEndpoint, RegistryError> {
        self.for_biz(biz_id)
            .filter(|endpoint| {
                endpoint.status.is_selectable()
                    && endpoint.protocols.supports(transport)
                    && endpoint.health.consecutive_failures < 3
            })
            .max_by_key(|endpoint| {
                (
                    endpoint.status.rank(),
                    endpoint.priority,
                    endpoint
                        .health
                        .latency_ms
                        .map_or(0, |latency| u64::MAX - latency),
                )
            })
            .ok_or_else(|| RegistryError::NoMatchingEndpoint {
                biz_id: biz_id.to_string(),
                transport,
            })
    }

    /// # [HANDLER] - EndpointRegistry::apply_probe_result
    /// * `DESC`: `将接入点测试结果写回本地状态`
    pub fn apply_probe_result(&mut self, result: ProbeResult) -> Result<(), RegistryError> {
        let endpoint = self
            .endpoints
            .iter_mut()
            .find(|endpoint| endpoint.id == result.endpoint_id)
            .ok_or_else(|| RegistryError::UnknownEndpoint(result.endpoint_id.clone()))?;

        endpoint.health = RouteHealth {
            last_test_ok: Some(result.ok),
            latency_ms: result.latency_ms,
            consecutive_failures: if result.ok {
                0
            } else {
                endpoint.health.consecutive_failures.saturating_add(1)
            },
            last_test_at_ms: Some(result.checked_at_ms),
        };
        if !result.ok && endpoint.status == EndpointStatus::Active {
            endpoint.status = EndpointStatus::Degraded;
        }
        if result.ok && endpoint.status == EndpointStatus::Degraded {
            endpoint.status = EndpointStatus::Active;
        }
        Ok(())
    }

    /// # [HANDLER] - EndpointRegistry::test
    /// * `DESC`: `通过可注入探测器测试指定业务的候选接入点`
    pub fn test<P: EndpointProbe>(
        &mut self,
        biz_id: &str,
        transport: TransportKind,
        probe: &P,
    ) -> Result<Vec<ProbeResult>, RegistryError> {
        let candidates: Vec<_> = self
            .for_biz(biz_id)
            .filter(|endpoint| endpoint.protocols.supports(transport))
            .map(|endpoint| {
                (
                    endpoint.id.clone(),
                    endpoint.address.clone(),
                    endpoint.protocols,
                )
            })
            .collect();

        if candidates.is_empty() {
            return Err(RegistryError::NoMatchingEndpoint {
                biz_id: biz_id.to_string(),
                transport,
            });
        }

        let mut results = Vec::with_capacity(candidates.len());
        for (id, address, protocols) in candidates {
            let result = probe.probe(&id, &address, protocols, transport);
            self.apply_probe_result(result.clone())?;
            results.push(result);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::endpoint::StaticProbe;

    #[test]
    fn builtin_table_contains_gateway_and_multiple_businesses() {
        let registry = EndpointRegistry::from_builtin();

        assert_eq!(registry.gateways().len(), 1);
        assert_eq!(registry.gateways()[0].port, 443);
        assert_eq!(registry.for_biz("im").count(), 2);
        assert_eq!(registry.for_biz("file-upload").count(), 1);
    }

    #[test]
    fn select_uses_biz_id_and_priority() {
        let registry = EndpointRegistry::from_builtin();
        let endpoint = registry.select("im", TransportKind::WebSocket).unwrap();

        assert_eq!(endpoint.id, "im-cn-east-1");
        assert_eq!(endpoint.address.connect_host, "203.0.113.10");
        assert_eq!(
            endpoint.address.tls_server_name.as_deref(),
            Some("im-edge.cola.example.com")
        );
    }

    #[test]
    fn failed_probe_degrades_endpoint_and_success_resets_it() {
        let mut registry = EndpointRegistry::from_builtin();
        let failed = StaticProbe::new(false, None, 100);
        registry
            .test("im", TransportKind::WebSocket, &failed)
            .unwrap();

        let first = registry
            .endpoints()
            .iter()
            .find(|item| item.id == "im-cn-east-1")
            .unwrap();
        assert_eq!(first.status, EndpointStatus::Degraded);
        assert_eq!(first.health.consecutive_failures, 1);

        let success = StaticProbe::new(true, Some(42), 200);
        registry
            .test("im", TransportKind::WebSocket, &success)
            .unwrap();
        let first = registry
            .endpoints()
            .iter()
            .find(|item| item.id == "im-cn-east-1")
            .unwrap();
        assert_eq!(first.status, EndpointStatus::Active);
        assert_eq!(first.health.consecutive_failures, 0);
        assert_eq!(first.health.latency_ms, Some(42));
    }
}

//////// END
