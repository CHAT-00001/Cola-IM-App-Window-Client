// cola_data/src/query.rs  -- 数据 - APP - 网关查询参数
// 2026/6/11 20:53

////////

use crate::auth::request::session::AuthSessionRequest;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;

////////

/// # [GLOBAL REQUEST] - APP网关统一请求上下文
/// * `desc` :采用“全可选/默认值”设计，单网关模式下各路由“各取所需”
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiGatewayRequest {
    // 1. 💡 基础
    pub node: Option<String>,      // 节点(预设,暂时无意义)
    pub code: Option<i16>,         // 代码(预设,暂时无意义)
    pub service: Option<String>,   // 子业务模块名称
    pub action: Option<i16>,       // 动作码 1000-9999
    pub uid: Option<i64>,          // 操作者 ID（URL或网关鉴权注入）
    pub req_id: Option<String>,    // 全局请求唯一 ID
    pub action_id: Option<String>, // 业务动作幂等 ID
    pub lang: Option<String>,      // 语言(可选)

    // 2. 📰 翻页
    pub page: Option<i64>, // 页码
    pub qty: Option<i64>,  // 每页数量

    // 3. 📍 位置
    pub lat: Option<f64>, // 纬度
    pub lng: Option<f64>, // 经度

    pub is: Option<bool>, // 是非

    // 4. 🆔 身份
    #[serde(default)]
    pub auth: Option<AuthSessionRequest>, // 验证信息(access_token / refresh_token ..)

    // 5. 🔢 业务专属参数（各取所需）
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
    pub user_id: i64, // 用户 ID
    #[serde(default)]
    pub dynamic_id: i64, // 动态 ID
    #[serde(default)]
    pub gift_id: i64, // 礼物 ID
    #[serde(default)]
    pub live_id: i64, // 直播 ID
    #[serde(default)]
    pub music_id: i64, // 音乐 ID
    #[serde(default)]
    pub album_id: Option<i64>, // 可选专辑 ID
    #[serde(default)]
    pub photo_id: i64, // 照片 ID
    #[serde(default)]
    pub video_id: i64, // 视频 ID
    #[serde(default)]
    pub category_id: i64, // 分类 ID
    #[serde(default)]
    pub city_id: i64, // 城市 ID
    #[serde(default)]
    pub comment_id: i64, // 评论 ID
    #[serde(default)]
    pub danmaku_id: i64, // 弹幕 ID
    #[serde(default)]
    pub keyword: String, // 关键词

    // 6. hashmap灵活扩展
    #[serde(default)]
    pub params: HashMap<String, String>,

    // 7. 后端计算辅助字段（不参与序列化）
    #[serde(skip, default)]
    pub limit: i64,
    #[serde(skip, default)]
    pub offset: i64,
    #[serde(skip, default)]
    pub poi_id: i64,

    // 8. 📦 原始请求体（Body JSON 透传，含 cmd 业务参数）
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

// 构造函数
impl ApiGatewayRequest {
    //

    ////////

    /// # [CASE] - 🚧 核心方法：构建并净化请求参数
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
        if self.user_id == 0 {
            if let Some(operator_id) = self.uid {
                self.user_id = operator_id;
            }
        }

        self
    }

    ////////

    /// # [CASE] - 🔗 字段级合并（Body 为主）
    /// * `desc` URL 解析出的基底 + Body 解析出的覆盖层
    /// * `合并规则`: Body 中**有值**的字段覆盖 URL，Body 中**缺省**的字段保留 URL
    pub fn merge(self, body: Self) -> Self {
        Self {
            // 1. 基础（Option 字段：body 有值才覆盖）
            node: body.node.or(self.node),
            code: body.code.or(self.code),
            service: body.service.or(self.service),
            action: body.action.or(self.action),
            uid: body.uid.or(self.uid),
            req_id: body.req_id.or(self.req_id),
            action_id: body.action_id.or(self.action_id),
            lang: body.lang.or(self.lang),

            // 2. 翻页
            page: body.page.or(self.page),
            qty: body.qty.or(self.qty),

            // 3. 位置
            lat: body.lat.or(self.lat),
            lng: body.lng.or(self.lng),

            // 4. 身份
            is: None,
            auth: body.auth.or(self.auth),

            // 5. 业务专属（默认值字段：body 非默认才覆盖）
            id: if body.id != 0 { body.id } else { self.id },
            _id: if body._id != 0 { body._id } else { self._id },
            at: if body.at != 0 { body.at } else { self.at },
            by: if !body.by.is_empty() {
                body.by
            } else {
                self.by
            },
            status: if body.status != 0 {
                body.status
            } else {
                self.status
            },
            user_id: if body.user_id != 0 {
                body.user_id
            } else {
                self.user_id
            },
            dynamic_id: if body.dynamic_id != 0 {
                body.dynamic_id
            } else {
                self.dynamic_id
            },
            gift_id: if body.gift_id != 0 {
                body.gift_id
            } else {
                self.gift_id
            },
            live_id: if body.live_id != 0 {
                body.live_id
            } else {
                self.live_id
            },
            music_id: if body.music_id != 0 {
                body.music_id
            } else {
                self.music_id
            },
            album_id: body.album_id.or(self.album_id),
            photo_id: if body.photo_id != 0 {
                body.photo_id
            } else {
                self.photo_id
            },
            video_id: if body.video_id != 0 {
                body.video_id
            } else {
                self.video_id
            },
            category_id: if body.category_id != 0 {
                body.category_id
            } else {
                self.category_id
            },
            city_id: if body.city_id != 0 {
                body.city_id
            } else {
                self.city_id
            },
            comment_id: if body.comment_id != 0 {
                body.comment_id
            } else {
                self.comment_id
            },
            danmaku_id: if body.danmaku_id != 0 {
                body.danmaku_id
            } else {
                self.danmaku_id
            },
            keyword: if !body.keyword.is_empty() {
                body.keyword
            } else {
                self.keyword
            },

            // 6. 灵活扩展
            params: if !body.params.is_empty() {
                body.params
            } else {
                self.params
            },

            // 7. 后端计算辅助字段（取 URL 的，build() 会重算）
            limit: self.limit,
            offset: self.offset,
            poi_id: self.poi_id,

            // 8. 原始请求体（Body 有值则覆盖）
            body: body.body.or(self.body),
        }
    }

    ////////

    /// # [CASE] - 🌐 空上下文快捷构造
    pub fn empty() -> Self {
        Self::default()
    }
}

//////// END
