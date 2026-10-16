// market/cmd/goods.rs
// data - MARKET - cmd - 商品
// 2026/6/18 13:28 Created.

////////

mod add;
mod view;

use crate::market::entity::goods::goods::GoodsEntity;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

////////

/// # [COMMAND] - 商品创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsCommand {
    pub name: String,               // 中文名称
    pub name_en: String,            // 英文名称
    pub no: String,                 // 货号
    pub one_classid: i64,           // 一级分类 ID
    pub two_classid: i64,           // 二级分类 ID
    pub three_classid: i64,         // 三级分类 ID
    pub price: String,              // 价格
    pub original_price: String,     // 原始价格
    pub thumbs: String,             // 封面图集
    pub content: String,            // 内容
    pub r#type: i16,                // 类型
    pub video_url: Option<String>,  // 视频地址
    pub video_length: Option<i16>,  // 视频长度
    pub commission: Option<String>, // 佣金
}

// 构造函数
impl Default for GoodsCommand {
    /// # 默认值兜底
    /// 当管理员只输入极少量信息时，构造一个标准的空商品模板
    fn default() -> Self {
        Self {
            name: "新商品".to_string(),
            name_en: "New Product".to_string(),
            no: "00-0000".to_string(),
            one_classid: 0,
            two_classid: 0,
            three_classid: 0,
            price: "0.00".to_string(),
            original_price: "0.00".to_string(),
            thumbs: "".to_string(),
            content: "".to_string(),
            r#type: 0,
            video_url: None,
            video_length: None,
            commission: Some("0.00".to_string()),
        }
    }
}

// 构造实现
impl GoodsCommand {
    /// # 映射到数据库实体
    /// 这里会自动填补所有必要的默认值，防止数据库写入失败
    pub fn to_entity(&self, admin_id: i64) -> GoodsEntity {
        let mut entity = GoodsEntity::default();

        // 唯一标识与时间戳（全面对齐新版本规范）
        entity._id = Some(Uuid::new_v4().to_string());

        let now: DateTime<Utc> = Utc::now();
        entity.created_at = Some(now);
        entity.updated_at = Some(now);

        // 兼容旧版整型时间戳
        let timestamp = now.timestamp() as i32;
        entity.add_time = timestamp;
        entity.upd_time = timestamp;

        // 核心业务字段
        entity.name = self.name.clone();
        entity.name_en = self.name_en.clone();

        // 三级分类完美对齐实体
        entity.one_classid = self.one_classid;
        entity.two_classid = self.two_classid;
        entity.three_classid = self.three_classid;

        // 价格字段映射（String -> Option<Decimal>）
        entity.price = Decimal::from_str(&self.price).ok();
        entity.present_price = Decimal::from_str(&self.price).ok();
        entity.original_price = Decimal::from_str(&self.original_price).ok();

        entity.thumbs = self.thumbs.clone();
        entity.content = self.content.clone();
        entity.r#type = self.r#type;

        // 填充兜底值或可选值
        entity.video_url = self.video_url.clone().unwrap_or_default();
        entity.video_length = self.video_length.map(|v| v as i32).unwrap_or_default();
        entity.commission = self.commission.clone();
        entity.admin_id = Some(admin_id.to_string());

        entity
    }

    /// # 数据校验
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("商品名称必填"));
        }
        if self.one_classid <= 0 {
            return Err(anyhow::anyhow!("一级分类必须选择"));
        }
        if self.price.parse::<f64>().unwrap_or(0.0) <= 0.0 {
            return Err(anyhow::anyhow!("商品价格必须大于0"));
        }
        Ok(())
    }
}

//////// END
