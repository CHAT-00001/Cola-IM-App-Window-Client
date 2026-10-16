// cola_data/src/user/cmd/vip.rs -- 数据 - USER - cmd - VIP 充值命令
// 2026/8/6 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - VIP 充值/开通命令
/// * `desc`: 用户开通或续费贵宾会员的命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VipCommand {
    pub id: i64,         // 目标用户ID（给谁开通）
    pub vip_type: i16,   // 贵宾类型：1=月度, 2=季度, 3=年度
    pub pay_method: i16, // 支付方式：1=微信, 2=支付宝, 3=苹果支付, 4=余额
    pub amount: i64,     // 支付金额（分）
    pub remark: String,  // 备注（用户昵称/订单号）
    pub source: String,  // 来源渠道：app, web, admin
}

//////// END
