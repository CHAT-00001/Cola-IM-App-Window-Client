// src/gateway/data/vo.rs -- 网关视图对象定义
// 2026/10/11 00:30 Created.

////////

pub mod gateway;

/// # [VO] - GatewayVo - 网关视图对象
pub struct GatewayVo {
    pub id: String,                  // 网关唯一标识
    pub name: String,                // 网关名称
    pub url: String,                 // 网关地址
    pub status: String,              // 网关状态
    pub description: Option<String>, // 网关描述
    pub created_at: String,          // 创建时间
    pub updated_at: String,          // 更新时间
}

//////// END
