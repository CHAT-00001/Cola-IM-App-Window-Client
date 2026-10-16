// cola_data/src/user/info/role.rs -- DATA - USER - info - 角色信息
// 2026/8/6 Created.

////////

use crate::cola_user::entity::role::UserRoleEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 角色信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: i64,                 // 角色 ID
    pub uid: i64,                // 操作者用户ID
    pub icon: Option<String>,    // 图标
    pub name: Option<String>,    // 英文名称
    pub name_zh: Option<String>, // 中文名称
    pub remark: Option<String>,  // 备注
    pub permission_level: i16,   // 权限等级: 1-9999
    pub status: i16,             // 状态码: 0无效 1有效
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// 构造与转换实现
impl RoleInfo {
    /// # 1. new - 构造一个新建角色的默认信息
    pub fn new(uid: i64, name: String, remark: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            uid,
            icon: None,
            name: Some(name.clone()),
            name_zh: Some(name),
            remark: Some(remark),
            permission_level: 2,
            status: 1,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    /// # 2. not_found - 构造一个表示“角色不存在”的空对象
    pub fn not_found() -> Self {
        Self {
            id: 0,
            uid: 0,
            icon: None,
            name: Some("Role Not Found".to_string()),
            name_zh: Some("角色不存在".to_string()),
            permission_level: 1,
            remark: Some("Role does not exist".to_string()),
            status: 0, // 0 代表无效/不存在
            created_at: None,
            updated_at: None,
        }
    }

    /// # 3. from_entity - 从底层数据库 Entity 转换
    /// *注：如果你的 Entity 结构字段类型不同，可在此处进行适配转换*
    pub fn from_entity(e: &UserRoleEntity) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            icon: e.icon.clone(),
            name: Some(e.name.clone()),
            name_zh: Some(e.name_zh.clone()),
            remark: e.remark.clone(),
            permission_level: e.permission_level,
            status: e.status,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

//////// END
