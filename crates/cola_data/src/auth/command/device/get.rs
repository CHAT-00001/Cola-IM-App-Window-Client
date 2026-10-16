// cola_data/src/code/cmd/node/get.rs  -- 数据 - AUTH - Command - 设备 - 获取命令
// 2026/6/26 12:04 Created.

//////

use serde::Deserialize;

//////

/// # [COMMAND] - 退出登录命令
/// * `device_id` 设备唯一标识 — 多端登录时只下线当前设备
#[derive(Debug, Deserialize, Clone)]
pub struct DeviceGetCommand {
    pub device_id: String, // 设备唯一标识
}

impl Default for DeviceGetCommand {
    fn default() -> Self {
        Self {
            device_id: "未知设备".to_string(),
        }
    }
}

////////
