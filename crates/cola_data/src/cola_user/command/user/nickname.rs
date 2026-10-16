// data/src/user/cmd/user/update.rs -- 用户资料 - 更新
// 2026/5/14 10:20

////////

use crate::cola_user::entity::user::UserEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户修改命令
/// * `desc`: 用户修改资料
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub nickname: Option<String>,          // 昵称
    pub signature: Option<String>,         // 个签
    pub avatar: Option<String>,            // 头像
    pub avatar_thumb: Option<String>,      // 小头像
    pub bg_img: Option<String>,            // 背景图
    pub sns_url: Option<String>,           // 社交网站
    pub email: Option<String>,             // 邮箱
    pub phone: Option<String>,             // 电话
    pub birthday: Option<i64>,             // 生日
    pub lat: Option<String>,               // 纬度
    pub lng: Option<String>,               // 经度
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

impl UpdateUserCommand {
    pub fn new() -> Self {
        Self {
            updated_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 将命令转换为用户实体，需要传入 user_id
    /// 注意：对于 Option 字段，如果为 None 则使用默认值（空字符串或0）
    pub fn to_entity(&self, user_id: i64) -> UserEntity {
        UserEntity {
            id: user_id,                             // 用户 ID
            _id: Option::from("".to_string()),       // UUID v4
            vx_id: Option::from("".to_string()),     // UUID v7
            user_type: None,                         // 用户类型
            user_nickname: self.nickname.clone(),    // 用户昵称
            signature: self.signature.clone(),       // 个性签名
            avatar: self.avatar.clone(),             // 头像
            avatar_thumb: self.avatar_thumb.clone(), // 小头像
            bg_img: self.bg_img.clone(),             // 背景图
            sns_url: self.sns_url.clone(),           // 社交网站
            email: self.email.clone(),               // 邮箱
            phone: self.phone.clone(),               // 电话
            birthday: self.birthday,                 // 生日
            last_login_time: None,                   // 最后登录时间(兼容旧版PHP)
            sex: None,                               // 性别
            perm_id: 0,                              // 权限 ID
            views: None,                             // 被浏览量
            likes: None,                             // 被点赞量
            fans: None,                              // 粉丝数量
            follows: None,                           // 关注数量
            level: None,                             // 关注等级
            author_level: None,                      // 主播等级
            lat: self.lat.clone(),                   // 纬度
            lng: self.lng.clone(),                   // 经度
            login_ip: Some("".to_string()),          // 登录 IP
            register_ip: Some("".to_string()),       // 注册 IP
            status: None,                            // 状态码
            is_deleted: None,                        // 逻辑删除
            create_time: 0,                          // 创建时间 (兼容旧版PHP)
            created_at: None,                        // 创建时间 (新版)
            updated_at: self.updated_at,             // 更新时间
            deleted_at: None,                        // 删除时间
            last_login_at: None,                     // 最后登录时间(新版)
            score: 0,                                // 默认积分
            coin: 1000000,                           // 钻石
            user_status: 1,                          // 状态码
        }
    }
}

//////// END
