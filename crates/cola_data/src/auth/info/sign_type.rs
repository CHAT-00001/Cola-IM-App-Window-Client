// cola_data/src/code/info/sign_type.rs  -- 数据中心 - AUTH - Info - 登录类型
// 2026/7/28 10:32

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 登录类型 元信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignTypeInfo {
    pub id: String,                // ID
    pub _id: String,               // 备用ID
    pub _sn: i64,                  // 雪花ID
    pub _a: String,                // uuidv4
    pub name: String,              // 名称
    pub name_zh: String,           // 中文名称
    pub icon: String,              // 图标
    pub thumb: String,             // 封面
    pub sort: i16,                 // 排序(默认9999)
    pub status: i16,               // 状态 0.禁用 1. 启用
    pub created_at: DateTime<Utc>, // 创建时间
    pub updated_at: DateTime<Utc>, // 更新时间
}

/// # [INFO] - 刷新ACCESS_TOKEN
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessTokenInfo {
    pub access_token: String,             // 临时 Token (JWT)
    pub access_expired_at: DateTime<Utc>, // 临时 Token 过期时间
}

//////// EDN
