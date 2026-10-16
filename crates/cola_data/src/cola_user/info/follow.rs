// cola_data/src/user/info/follow.rs
// 可乐数据中心 - 用户 - info - 用户信息
// 2026/8/10 05:53 Created.

////////

use crate::cola_user::entity::user::UserEntity;
use chrono::{Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户关注信息
/// * `desc`: `用户资料(可缓存)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollowInfo {
    pub id: i64,                   // id
    pub nickname: String,          // 昵称
    pub avatar_url: String,        // 头像
    pub bg_img: String,            // 背景图
    pub signature: Option<String>, // 签名
    pub birthday: Option<i32>,     // 生日（时间戳，兼容原逻辑采用 i32）
    pub add_time: i64,             // 兼容PHP（时间戳，内部用 i64 承载）
    pub status: i16,               // 状态
    pub age: Option<i16>,          // 年龄（动态计算）

    // 🚀 新增动态字段
    pub is_following: bool, // 是否关注
    pub is_online: bool,    // 是否在线
    pub is_streaming: bool, // 是否在直播
}

impl UserFollowInfo {
    /// 构造一个带有默认值的 UserInfo，并根据生日动态计算年龄
    pub fn new(birthday: Option<i32>) -> Self {
        let current_time = Utc::now().timestamp();

        // 动态计算年龄
        let age = match birthday {
            Some(b_timestamp) => {
                let birth_date = Utc.timestamp_opt(b_timestamp as i64, 0).single();
                let now_date = Utc::now();

                match birth_date {
                    Some(birth) => {
                        let mut calculated_age = now_date.year() - birth.year();

                        if now_date.month() < birth.month()
                            || (now_date.month() == birth.month() && now_date.day() < birth.day())
                        {
                            calculated_age -= 1;
                        }

                        Some(calculated_age.max(0) as i16)
                    }
                    None => Some(0),
                }
            }
            None => Some(0),
        };

        Self {
            id: 0,
            nickname: "用户不存在".to_string(),
            avatar_url: "".to_string(),
            bg_img: "".to_string(),
            signature: Some("这里还是空的啊~".to_string()),
            birthday,
            status: 0,
            add_time: current_time,
            age,

            is_following: false,
            is_online: false,
            is_streaming: false,
        }
    }
}

// 同时实现 Default Trait，方便直接使用 UserInfo::default()
impl Default for UserFollowInfo {
    fn default() -> Self {
        Self::new(None)
    }
}

////////

/// # 🚀 【精准对齐】物理实体到视图模型的转化契约
/// * 机制：处理 Option 解包，安全地将 Entity 属性 1:1 投射到 UI 视图层
impl From<UserEntity> for UserFollowInfo {
    fn from(entity: UserEntity) -> Self {
        // 1. 将实体的 i64 生日安全缩减为 i32 传给基础年龄计算基座
        let birthday_i32 = entity.birthday.map(|b| b as i32);
        let mut info = Self::new(birthday_i32);

        // 2. 严格对齐你最新的 UserEntity 字段契约
        info.id = entity.id; // 👈 匹配实体主键 id

        info.nickname = entity
            .user_nickname
            .unwrap_or_else(|| "一罐可乐".to_string()); // 👈 对齐 user_nickname

        info.avatar_url = entity.avatar.unwrap_or_default(); // 👈 对齐 avatar
        info.bg_img = entity.bg_img.unwrap_or_default(); // 👈 对齐 bg_img

        // 如果实体有签名则覆盖，没有则保留基座默认的 "这里还是空的啊~"
        if entity.signature.is_some() {
            info.signature = entity.signature; // 👈 对齐 signature
        }

        info.status = entity.status.unwrap_or(1); // 👈 对齐 status
        info.add_time = entity.create_time as i64; // 👈 完美强转兼容 PHP 的 i32 时间戳

        // 3. 社交状态默认值，等待 Service 层后续通过 Redis 旁路染色填充
        info.is_following = false;
        info.is_online = false;
        info.is_streaming = false;

        info
    }
}

//////// END
