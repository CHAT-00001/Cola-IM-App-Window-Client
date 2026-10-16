// cola_data/src/user/cmd/user/role.rs -- 数据 - USER - cmd - 角色命令
// 2026/9/5 08:04 Created.

////////

use crate::cola_user::entity::role::UserRoleEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户角色创建命令
/// * `desc`: `管理员添加新角色`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserRoleCreateCmd {
    pub uid: i64,               // 操作者用户ID
    pub group_id: Option<i64>,  // 所属用户组ID
    pub icon: Option<String>,   // 图标
    pub name: String,           // 英文名称/标识
    pub name_zh: String,        // 中文名称
    pub remark: Option<String>, // 备注
    pub permission_level: i16,  // 权限等级: 1-9999
}

impl UserRoleCreateCmd {
    pub fn new() -> Self {
        Self::default()
    }

    /// 将创建命令转换为角色实体
    pub fn to_entity(&self) -> UserRoleEntity {
        let now = Utc::now();
        UserRoleEntity {
            id: 0, // 新增时由数据库自增生成
            uid: self.uid,
            group_id: self.group_id,
            icon: self.icon.clone(),
            name: self.name.clone(),
            name_zh: self.name_zh.clone(),
            remark: self.remark.clone(),
            permission_level: self.permission_level.clamp(1, 9999),
            status: 1,         // 默认有效
            is_deleted: false, // 默认未删除
            created_at: Some(now),
            updated_at: Some(now),
            deleted_at: None,
        }
    }
}

////////

/// # [COMMAND] - 用户角色更新命令
/// * `desc`: `管理员更新角色资料`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserRoleUpdateCmd {
    pub id: i64,                       // 角色ID（必须）
    pub group_id: Option<i64>,         // 所属用户组ID
    pub icon: Option<String>,          // 图标
    pub name: Option<String>,          // 英文名称/标识
    pub name_zh: Option<String>,       // 中文名称
    pub remark: Option<String>,        // 备注
    pub status: Option<i16>,           // 状态码: 0无效 1有效
    pub permission_level: Option<i16>, // 权限等级: 1-9999
}

impl UserRoleUpdateCmd {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// 将更新命令应用到现有的实体上（增量合并更新）
    pub fn apply_to_entity(&self, entity: &mut UserRoleEntity) {
        if let Some(group_id) = self.group_id {
            entity.group_id = Some(group_id);
        }
        if let Some(ref icon) = self.icon {
            entity.icon = Some(icon.clone());
        }
        if let Some(ref name) = self.name {
            entity.name = name.clone();
        }
        if let Some(ref name_zh) = self.name_zh {
            entity.name_zh = name_zh.clone();
        }
        if let Some(ref remark) = self.remark {
            entity.remark = Some(remark.clone());
        }
        if let Some(status) = self.status {
            entity.status = status;
        }
        if let Some(permission_level) = self.permission_level {
            entity.permission_level = permission_level.clamp(1, 9999);
        }
        entity.updated_at = Some(Utc::now());
    }
}

//////// END
