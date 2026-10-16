// user/cmd/category.rs
// 用户 - cmd - 分类
// 2026/8/4 01:58 Created.

////////

use crate::cola_user::entity::category::UserCategoryEntity; // 假设对应分类实体
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户 分类命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCategoryCommand {
    pub name: Option<String>,        // 分类名称
    pub sort: Option<i32>,           // 排序
    pub description: Option<String>, // 描述
    pub icon: Option<String>,        // 图标
}

impl UserCategoryCommand {
    /// 💡 显式提供一个快捷的 new 方法（利用 Default）
    pub fn new() -> Self {
        Self::default()
    }

    /// 将命令中的变更应用到现有的分类实体上
    pub fn apply(self, mut entity: UserCategoryEntity) -> UserCategoryEntity {
        if let Some(v) = self.name {
            entity.name = v;
        }
        if let Some(v) = self.sort {
            entity.sort = v;
        }
        if let Some(v) = self.description {
            entity.description = Some(v);
        }
        if let Some(v) = self.icon {
            entity.icon = Some(v);
        }

        entity
    }
}

//////// END
