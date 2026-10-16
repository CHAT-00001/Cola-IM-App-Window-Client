// cola_data/src/app/request.rs  -- DATA - APP - 请求体
// 2026/4/16 07:45

use std::cmp;

////////

/// # [PARAMS] - URL请求参数
/// * 从URL Params 获取
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiUrlParamsQuery {
    pub uid: Option<i64>,       // 操作者 ID
    pub req_id: Option<String>, // 请求ID
    pub page: Option<i64>,      // 页码
    pub qty: Option<i64>,       // 数量
    pub lat: Option<f64>,       // 纬度
    pub lng: Option<f64>,       // 经度

    // 计算后的分页字段（不参与URL参数解析）
    #[serde(skip, default)]
    pub limit: i64,

    #[serde(skip, default)]
    pub offset: i64,
}

/// # 构造API列表请求体
impl ApiUrlParamsQuery {
    /// 新的
    pub fn new(uid: Option<i64>, page: i64, qty: i64) -> Self {
        // 页码：最小 1
        let final_page = cmp::max(page, 1);
        // 数量：最小 1，最大 50
        let final_qty = cmp::min(cmp::max(qty, 1), 50);

        // 计算分页
        let limit = final_qty;
        let offset = (final_page - 1) * final_qty;

        Self {
            uid,
            req_id: None,
            page: Some(final_page),
            qty: Some(final_qty),
            limit,
            offset,
            lat: None,
            lng: None,
        }
    }

    /// 推荐用法：从 URL 参数自动解析 + 自动处理
    pub fn build(self) -> Self {
        let page = self.page.unwrap_or(1);
        let qty = self.qty.unwrap_or(10);

        Self::new(self.uid, page, qty)
    }
}

//////// END
