// cola_data/src/code/cmd/phone.rs  -- 可乐数据中心 - AUTH - Command - 手机短信验证码登录验证器
// 2026/4/13 00:49 by wx: cestbon10080

//////

use serde::{Deserialize, Serialize};
use validator::Validate;

//////

/// # [COMMAND] - 手机验证码登录
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct PhoneLoginCommand {
    #[serde(default = "default_area_code")]
    pub area_code: String, // 地区编号

    #[validate(length(min = 7, max = 15, message = "手机号格式不正确"))] // 国际化通常支持到15位
    pub phone_no: String, // 电话号码

    #[validate(length(min = 4, max = 8, message = "验证码长度非法"))]
    pub code: String, // 短信验证码

    #[serde(default = "default_device_id")] // 增加默认值配置
    pub device_id: String, // 设备唯一标识 (用于多设备会话管理)

    #[serde(default = "default_platform")] // 增加默认值配置
    pub platform: String, // 平台标识，如: "ios", "android", "web"

    #[serde(default)]
    pub client_ip: String, // 客户端IP（网关层从HttpRequest提取，不参与客户端参数校验）
}

// 默认地区编码 0086
fn default_area_code() -> String {
    "0086".to_string()
}
fn default_device_id() -> String {
    "未知设备".to_string()
}
fn default_platform() -> String {
    "未知平台".to_string()
}

// 构造实现 1
// 为处理空字符串的情况，建议实现 Default trait
impl Default for PhoneLoginCommand {
    fn default() -> Self {
        Self {
            area_code: default_area_code(), // 地区码
            phone_no: "".to_string(),       // 电话号码
            code: "".to_string(),           // 短信验证码
            device_id: default_device_id(), // 设备 ID
            platform: default_platform(),   // 平台
            client_ip: String::new(),       // 客户端 IP
        }
    }
}

// 构造实现 2
impl PhoneLoginCommand {
    //

    ////////

    /// # 1. [CASE] - 快速校验接口
    /// * `desc`: 在 Case 层直接调用此方法，确保输入参数符合业务规则
    pub fn validate_params(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }

    ////////

    /// # 2. [CASE] - 辅助方法：
    /// * `desc`: 生成一个脱敏的日志记录，保护用户隐私
    /// * `desc`: 在记录登录日志时，不要打印完整的手机号或验证码
    pub fn to_log_info(&self) -> String {
        format!(
            "platform: {}, node: {}, phone: {}***{}, ip: {}",
            self.platform,
            self.device_id,
            &self.phone_no[..3],
            &self.phone_no[self.phone_no.len() - 2..],
            self.client_ip,
        )
    }
}

//////// END
