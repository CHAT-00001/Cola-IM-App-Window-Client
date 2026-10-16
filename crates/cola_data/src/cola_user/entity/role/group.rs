// cola_data/src/user/entity/role/group.rs -- DATA - USER - entity - 角色 - 用户组表
// 2026/9/30 05:40 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 角色表
/// * `pg schema`: `cola_user` -- PG 模式
/// * `table name`: `role_group` - 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserGroupEntity {
    pub id: i64,                           // 主键ID
    pub uid: i64,                          // 创建者/操作者用户ID
    pub group_id: Option<i64>,             // 所属用户组ID（如需关联用户组）
    pub icon: Option<String>,              // 图标
    pub name: String,                      // 英文标识/编码（如: admin, editor）
    pub name_zh: String,                   // 中文名称（如: 管理员, 编辑）
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态码: 0无效 1有效
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_GRPOUP_COLUMNS: &str = r#"
    id, uid, group_id, icon, name, name_zh, remark,
    status, is_deleted,
    created_at, updated_at, deleted_at
"#;

//////// END
