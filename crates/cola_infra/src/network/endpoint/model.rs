// src/endpoint/model.rs -- Endpoint and gateway data models
// 2026/10/9 00:25 Created.

////////

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EndpointStatus {
    Active,
    Degraded,
    Draining,
    Maintenance,
    Disabled,
    Unknown,
}

impl EndpointStatus {
    pub(crate) fn is_selectable(self) -> bool {
        matches!(self, Self::Active | Self::Degraded)
    }

    pub(crate) fn rank(self) -> u8 {
        match self {
            Self::Active => 5,
            Self::Degraded => 3,
            Self::Draining => 1,
            Self::Maintenance | Self::Disabled | Self::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TransportKind {
    Http,
    WebSocket,
    WebTransport,
    Grpc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EndpointProtocol {
    pub http: bool,
    pub websocket: bool,
    pub webtransport: bool,
    pub grpc: bool,
}

impl EndpointProtocol {
    pub const fn http_websocket() -> Self {
        Self {
            http: true,
            websocket: true,
            webtransport: false,
            grpc: false,
        }
    }

    pub const fn supports(self, transport: TransportKind) -> bool {
        match transport {
            TransportKind::Http => self.http,
            TransportKind::WebSocket => self.websocket,
            TransportKind::WebTransport => self.webtransport,
            TransportKind::Grpc => self.grpc,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EndpointAddress {
    pub connect_host: String,
    pub port: u16,
    pub tls_server_name: Option<String>,
    pub authority: Option<String>,
}

impl EndpointAddress {
    pub fn ip_with_sni(
        connect_host: impl Into<String>,
        port: u16,
        tls_server_name: impl Into<String>,
    ) -> Self {
        let server_name = tls_server_name.into();
        Self {
            connect_host: connect_host.into(),
            port,
            authority: Some(server_name.clone()),
            tls_server_name: Some(server_name),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuiltinRoute {
    pub endpoint_id: &'static str,
    pub biz_id: &'static str,
    pub priority: u16,
    pub status: EndpointStatus,
    pub address: EndpointAddress,
    pub protocols: EndpointProtocol,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuiltinGateway {
    pub id: &'static str,
    pub domain: &'static str,
    pub port: u16,
    pub refresh_path: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayEndpoint {
    pub id: String,
    pub biz_id: String,
    pub priority: u16,
    pub status: EndpointStatus,
    pub address: EndpointAddress,
    pub protocols: EndpointProtocol,
    pub health: RouteHealth,
}

impl GatewayEndpoint {
    pub fn from_builtin(route: BuiltinRoute) -> Self {
        Self {
            id: route.endpoint_id.to_string(),
            biz_id: route.biz_id.to_string(),
            priority: route.priority,
            status: route.status,
            address: route.address,
            protocols: route.protocols,
            health: RouteHealth::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct RouteHealth {
    pub last_test_ok: Option<bool>,
    pub latency_ms: Option<u64>,
    pub consecutive_failures: u32,
    pub last_test_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProbeResult {
    pub endpoint_id: String,
    pub transport: TransportKind,
    pub ok: bool,
    pub latency_ms: Option<u64>,
    pub checked_at_ms: u64,
    pub message: Option<String>,
}

//////// END
