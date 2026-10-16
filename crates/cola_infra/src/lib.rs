// cola_infra/src/lib.rs -- INFRA - lib
// 2026/10/9 00:20 Created.

////////

pub mod abi; // 应用进程接口
pub mod client;
pub mod config;
pub mod error;
pub mod handler; // 接口路径与方法定义
pub mod im;
pub mod model;
pub mod network; // 网络层 (接入点 / HTTP)
pub mod port;
pub mod storage;
pub mod ui_api;
mod util; // 工具

//////// END

