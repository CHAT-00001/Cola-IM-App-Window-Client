// src/endpoint/mod.rs -- Endpoint registry public facade
// 2026/10/9 00:25 Created.

mod model;
mod probe;
mod registry;

pub use model::{
    BuiltinGateway, BuiltinRoute, EndpointAddress, EndpointProtocol, EndpointStatus,
    GatewayEndpoint, ProbeResult, RouteHealth, TransportKind,
};
pub use probe::{EndpointProbe, StaticProbe};
pub use registry::{EndpointRegistry, RegistryError};

////////

/// # [HANDLER] - builtin_registry
/// * `DESC`: `创建包含全球网关和业务兜底接入点的默认注册表`
pub fn builtin_registry() -> EndpointRegistry {
    EndpointRegistry::from_builtin()
}

//////// END
