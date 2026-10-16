// cola_data/src/user/entity/role.rs -- DATA - USER - entity - 角色表
// 2026/8/3 14:45 Created.

////////

pub mod group; // 用户组
pub mod permission; // 角色权限

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 角色表
/// * `pg schema`: `cola_user`
/// * `table name`: `role`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserRoleEntity {
    pub id: i64,                           // 主键ID
    pub uid: i64,                          // 创建者/操作者用户ID
    pub group_id: Option<i64>,             // 所属用户组ID（如需关联用户组）
    pub icon: Option<String>,              // 图标
    pub name: String,                      // 英文标识/编码（如: admin, editor）
    pub name_zh: String,                   // 中文名称（如: 管理员, 编辑）
    pub remark: Option<String>,            // 备注
    pub permission_level: i16,             // 权限等级: 1-9999
    pub status: i16,                       // 状态码: 0无效 1有效
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_ROLE_COLUMNS: &str = r#"
    id, uid, group_id, icon, name, name_zh, remark,
    status, is_deleted, permission_level,
    created_at, updated_at, deleted_at
"#;

//////// END
