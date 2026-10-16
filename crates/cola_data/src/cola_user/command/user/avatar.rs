// cola_data/src/user/cmd/user/avatar.rs -- DATA - USER - 资料 - 头像
// 2026/5/14 10:20

////////

use crate::cola_user::entity::user::avatar::UserAvatarEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户 - 更新头像
/// * `desc`: 用户修改资料
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvaterUpdateCommand {
    pub avatar: Option<String>,            // 头像
    pub avatar_thumb: Option<String>,      // 小头像
    pub bg_img: Option<String>,            // 背景图
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

impl AvaterUpdateCommand {
    pub fn new() -> Self {
        Self {
            updated_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 将命令转换为用户实体，需要传入 user_id
    /// 注意：对于 Option 字段，如果为 None 则使用默认值（空字符串或0）
    pub fn to_entity(&self, user_id: i64) -> UserAvatarEntity {
        UserAvatarEntity {
            id: user_id,                             // 用户 ID
            _id: Option::from("".to_string()),       // UUID v4
            user_id,                                 // 用户 ID
            avatar: self.avatar.clone(),             // 头像
            avatar_thumb: self.avatar_thumb.clone(), // 小头像
            bg_img: self.bg_img.clone(),             // 背景图
            views: 0,                                // 被浏览量
            likes: 0,                                // 被点赞量
            status: None,                            // 状态码
            is_deleted: None,                        // 逻辑删除
            is_public: None,                         // 是否公开
            created_at: None,                        // 创建时间
            updated_at: self.updated_at,             // 更新时间
            deleted_at: None,                        // 删除时间
            last_pick_at: None,
        }
    }
}

/// # [COMMAND] - 用户 - 选择头像
/// * `desc`: `用户侧 - 从历史记录中选择头像`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvaterPickCommand {
    pub id_key: String,                    // 操作幂等键
    pub avatar_id: i64,                    // 头像 ID
    pub avatar_thumb: Option<String>,      // 小头像
    pub bg_img: Option<String>,            // 背景图
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

//////// END
