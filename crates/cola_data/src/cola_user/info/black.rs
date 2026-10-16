// cola_data/src/user/info/black.rs
// 可乐数据中心 - USER - info - 黑名单信息
// 2026/8/7 03:08 Created.

////////

use crate::cola_user::entity::black::UserBlackEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 黑名单信息
/// * `desc`: `黑名单的操作记录信息，用于前端审计与展示`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBlackInfo {
    pub id: i64,                           // 黑名单记录ID
    pub _id: Option<String>,               // UUID v4
    pub uid: i64,                          // 操作者用户ID
    pub user_id: i64,                      // 目标用户ID
    pub name: Option<String>,              // 名字
    pub remark: Option<String>,            // 备注
    pub is_deleted: bool,                  // 是否删除
    pub status: i16,                       // 状态码: 0无效 1有效
    pub add_time: i64,                     // 添加时间（时间戳）
    pub upd_time: i64,                     // 更新时间（时间戳）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

impl UserBlackInfo {
    /// 构造一个带有默认值的 UserBlackInfo 占位基座
    pub fn new() -> Self {
        let current_time = Utc::now().timestamp();

        Self {
            id: 0,
            _id: None,
            uid: 0,
            user_id: 0,
            name: None,
            remark: None,
            is_deleted: false,
            status: 1,
            add_time: current_time,
            upd_time: current_time,
            created_at: None,
            updated_at: None,
        }
    }
}

/// 默认实现
impl Default for UserBlackInfo {
    fn default() -> Self {
        Self::new()
    }
}

////////

/// # 🚀 【精准对齐】物理实体到视图模型的转化契约
/// * 机制：安全地将 UserBlackEntity 属性 1:1 投射到 UI 视图层
impl From<UserBlackEntity> for UserBlackInfo {
    fn from(entity: UserBlackEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            uid: entity.uid,
            user_id: entity.user_id,
            name: entity.name,
            remark: entity.remark,
            is_deleted: entity.is_deleted,
            status: entity.status,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//////// END
