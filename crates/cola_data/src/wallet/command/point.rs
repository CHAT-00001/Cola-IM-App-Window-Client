// cola_data/src/wallet/cmd/point.rs
// ✅ WALLET - 新用户积分账户初始化命令
// 2026/8/20 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 初始化用户积分账户
/// * `desc`: `创建 POINT 账户；当 initial_points 大于 0 时同步生成首笔赠送交易`
/// * `condition`: `积分最小单位为 1；idempotency_key 用于防止注册重试重复赠送`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPointInitCommand {
    pub user_id: i64,            // 用户ID
    pub initial_points: i64,     // 初始赠送积分，最小单位为1
    pub idempotency_key: String, // 初始化幂等键
    pub business_type: String,   // 业务类型
    pub business_id: String,     // 业务单号
    pub remark: Option<String>,  // 交易备注
}

////////

impl WalletPointInitCommand {
    /// # 1. [COMMAND] - 构建新用户积分账户初始化命令
    /// * `desc`: `使用 user_id 生成稳定幂等键，确保注册重试不会重复赠送积分`
    pub fn new(user_id: i64, initial_points: i64) -> Self {
        Self {
            user_id,
            initial_points,
            idempotency_key: format!("user_register:point:{user_id}"),
            business_type: "USER_REGISTER".to_string(),
            business_id: user_id.to_string(),
            remark: Some("新用户注册积分赠送".to_string()),
        }
    }

    ////////

    /// # 2. [COMMAND] - 校验积分初始化命令
    /// * `desc`: `校验用户ID、积分数和幂等键`
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.user_id <= 0 {
            return Err(anyhow::anyhow!("用户ID必须大于0"));
        }
        if self.initial_points < 0 {
            return Err(anyhow::anyhow!("初始积分不能为负数"));
        }
        if self.initial_points > 0 && self.idempotency_key.trim().is_empty() {
            return Err(anyhow::anyhow!("积分赠送必须提供幂等键"));
        }
        Ok(())
    }
}

//////// END
