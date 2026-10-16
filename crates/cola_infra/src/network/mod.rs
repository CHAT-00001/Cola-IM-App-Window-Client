// src/network/mod.rs -- 网络层 - mod (原 gate, 全球网关/接入点)
// 2026/10/16 06:50 Created.

////////

pub mod domain; // 域名 (sni 和 公网域名)
pub mod endpoint; // 接入点注册表与探测
pub mod http; // HTTP 请求发送 (基于接入点)
mod node; // 节点(备用)
mod port;

//////// END
