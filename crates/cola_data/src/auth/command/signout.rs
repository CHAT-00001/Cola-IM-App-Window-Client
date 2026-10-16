// cola_data/src/code/cmd/signout.rs  -- 可乐数据中心 - AUTH - Command - 退出登录
// 2026/6/26

//////

use serde::Deserialize;

//////

/// # [COMMAND] - 退出登录命令
/// * `device_id` 设备唯一标识 — 多端登录时只下线当前设备
#[derive(Debug, Deserialize, Clone)]
pub struct SignOutCommand {
    pub device_id: String, // 设备唯一标识
}

impl Default for SignOutCommand {
    fn default() -> Self {
        Self {
            device_id: "未知设备".to_string(),
        }
    }
}

////////
