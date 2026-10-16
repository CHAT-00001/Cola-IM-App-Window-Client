// data/src/code/info/file.rs
// 数据 - AUTH - INFO - 身份识别
// 2026/8/14 10:14 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// # [DTO] - 验证中心 - 身份信息脱敏视图
/// 用于 API 响应，确保敏感信息不泄露
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityInfo {
    // 使用 String 暴露 UUID，隐藏数据库内部的自增 ID
    pub id: String,
    pub user_id: i64,

    // 身份类型与状态，建议在 DTO 层保持清晰
    pub id_type: i16,
    pub status: i16,

    // 脱敏后的标识符（例如：手机号脱敏 '138****8888'）
    pub identifier: String,

    // 仅暴露验证状态和关键时间点
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 实现从 Entity 到 DTO 的转换
impl From<crate::auth::entity::identity::IdentityEntity> for IdentityInfo {
    fn from(entity: crate::auth::entity::identity::IdentityEntity) -> Self {
        Self {
            // 优先使用 UUID，如果不存在则将内部 id 转为字符串
            id: entity._id.unwrap_or_else(|| entity.id.to_string()),
            user_id: entity.user_id,
            id_type: entity.id_type.unwrap_or(0),
            status: entity.status,
            // 可以在此处实现脱敏逻辑
            identifier: mask_identifier(entity.identifier.unwrap_or_default()),
            is_verified: entity.verified_at.is_some(),
            created_at: entity.created_at.unwrap_or_else(Utc::now),
            updated_at: entity.updated_at,
        }
    }
}

/// 简单的脱敏函数示例
fn mask_identifier(id: String) -> String {
    if id.len() > 7 {
        format!("{}****{}", &id[0..3], &id[id.len() - 4..])
    } else {
        "***".to_string()
    }
}
