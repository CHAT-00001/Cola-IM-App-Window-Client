// cola_data/src/code/cmd/service/refresh.rs  -- 数据 - AUTH - Command - 会话 - 刷新命令
// 2026/6/9 07:39 Created.

use serde::Deserialize;
use validator::Validate;

////////

/// # [COMMAND] - 会话刷新命令
/// * `DESC`: `客户端根据设备ID和refresh_token定期刷新access_token`
#[derive(Debug, Default, Deserialize, Validate)]
pub struct SessionRefreshCommand {
    #[validate(length(min = 1, message = "refresh_token 不能为空"))]
    pub refresh_token: String, // Token
    #[validate(length(min = 1, message = "device_id 不能为空"))]
    pub device_id: String, // 设备 ID
    pub platform: i32, // 平台
}

////////

impl SessionRefreshCommand {
    /// 快速校验接口
    /// 在 Case 层直接调用此方法，确保输入参数符合业务规则
    pub fn validate_params(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }

    /// 辅助方法：生成一个脱敏的日志记录，保护敏感 Token
    pub fn to_log_info(&self) -> String {
        let masked_token = if self.refresh_token.len() > 8 {
            format!(
                "{}***{}",
                &self.refresh_token[..4],
                &self.refresh_token[self.refresh_token.len() - 4..]
            )
        } else {
            "***".to_string()
        };

        format!(
            "platform: {}, node: {}, refresh_token: {}",
            self.platform, self.device_id, masked_token
        )
    }
}

//////// END
