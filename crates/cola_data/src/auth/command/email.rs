// cola_data/src/code/cola_dynamic/email.rs  -- 可乐数据中心 - 验证中心 - Command - 邮箱验证码登录
// 2026/6/9 07:39

////////

use serde::Deserialize;
use validator::Validate;

////////

/// # [COMMAND] - 邮箱验证码登录命令
#[derive(Debug, Default, Deserialize, Validate)]
pub struct EmailLoginCommand {
    pub email: String,     // 邮箱
    pub code: String,      // 验证码
    pub device_id: String, // 设备ID
    pub platform: i32,     // 平台
}

////////

// 默认地区编码 0086
fn default_area_code() -> String {
    "0086".to_string()
}

impl EmailLoginCommand {
    /// 快速校验接口
    /// 在 Case 层直接调用此方法，确保输入参数符合业务规则
    pub fn validate_params(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }

    /// 辅助方法：生成一个脱敏的日志记录，保护用户隐私
    /// 在记录登录日志时，不要打印完整的手机号或验证码
    pub fn to_log_info(&self) -> String {
        format!(
            "platform: {}, node: {}, phone: {}***{}",
            self.platform,
            self.device_id,
            &self.email[..3],
            &self.code[self.code.len() - 2..]
        )
    }
}
