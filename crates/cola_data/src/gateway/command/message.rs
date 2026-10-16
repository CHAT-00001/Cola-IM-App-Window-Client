// data/src/gateway/cmd/message.rs  -- 数据中心 - GATEWAY - cmd - message.rs
// 2026/7/22 01:58

////////

use crate::auth::info::auth::AuthContext;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;
use std::net::IpAddr;

////////

/// # [GATEWAY MESSAGE SHELL] - 网关中心 请求消息体
/// * `desc` :采用“全可选/默认值”设计，单网关模式下各路由“各取所需”
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GatewayMessage {
    // 1. 💡 基础
    pub node: Option<i16>,       // 节点
    pub service: Option<String>, // 服务名称
    pub action: Option<i16>,     // 动作码
    pub uid: Option<i64>,        // 操作者 ID（URL或网关鉴权注入）
    pub req_id: Option<String>,  // 全局请求唯一 ID

    // 2. 📰 翻页
    pub page: Option<i64>, // 页码
    pub qty: Option<i64>,  // 每页数量
    // 3. 📍 位置
    pub lat: Option<f64>, // 纬度
    pub lng: Option<f64>, // 经度

    // 🆔 验证中心
    #[serde(skip)]
    pub auth: AuthContext,

    // 3. 公共参数(可选)
    #[serde(default)]
    pub id: i64, // ID
    #[serde(default)]
    pub _id: i64, // 雪花 ID
    #[serde(default)]
    pub at: i64, // 属于
    #[serde(default)]
    pub by: String, // 搭载于
    #[serde(default)]
    pub status: i16, // 状态码
    #[serde(default)]
    pub to: i64, // 到达 ID
    #[serde(default)]
    pub dynamic_id: i64, // 动态 ID
    #[serde(default)]
    pub keyword: String, // 关键词

    // 4. hashmap灵活扩展
    #[serde(default)]
    pub params: HashMap<String, String>,

    // 5. 后端计算辅助字段（不参与序列化）
    #[serde(skip, default)]
    pub limit: i64,
    #[serde(skip, default)]
    pub offset: i64,
}

// 构造函数
impl GatewayMessage {
    ////////

    /// # 1. [CASE] - 🚧 核心方法：构建并净化请求参数
    /// * `desc` 网关反序列化完成后，调用此方法初始化分页、权限等安全边界
    pub fn build(mut self) -> Self {
        // 1. 处理分页默认值与边界安全
        let raw_page = self.page.unwrap_or(1);
        let raw_qty = self.qty.unwrap_or(10); // 默认每页 10 条

        let final_page = cmp::max(raw_page, 1);
        let final_qty = cmp::min(cmp::max(raw_qty, 1), 50); // 最大限制 50 条

        self.page = Some(final_page);
        self.qty = Some(final_qty);
        self.limit = final_qty;
        self.offset = (final_page - 1) * final_qty;

        // 2. 如果网关已经校验了登录，可以把 uid 顺手同步给 user_id（视业务而定）
        if self.uid == Some(0) {
            if let Some(operator_id) = self.uid {
                self.uid = Option::from(operator_id);
            }
        }

        self
    }

    ////////

    /// # 2. [CASE] - 🌐 空上下文快捷构造
    pub fn empty() -> Self {
        Self::default()
    }
}

///////// END
