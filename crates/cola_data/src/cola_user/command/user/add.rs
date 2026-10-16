// cola_data/src/user/cmd/user/code -- DATA - USER - Command - 用户 - 发布
// 2026/5/22 16:35 Created.

////////

use crate::cola_user::entity::user::UserEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # [COMMAND] - 用户创建与修改命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCommand {
    pub _id: Option<String>,           // UUID v4（可选，不传服务端生成）
    pub vx_id: Option<String>,         // UUID v7（模拟微信ID）
    pub user_type: i16,                // 用户类型: 默认 2 (普通用户)
    pub nickname: Option<String>,      // 用户昵称
    pub signature: Option<String>,     // 个性签名（修改时可选）
    pub avatar: Option<String>,        // 头像（修改时可选）
    pub avatar_thumb: Option<String>,  // 小头像
    pub bg_img: Option<String>,        // 背景图（修改时可选）
    pub sns_url: Option<String>,       // 社交网络地址
    pub email: Option<String>,         // 邮箱
    pub phone: Option<String>,         // 电话号码 （带地区号，比如 008600000000000）
    pub birthday: Option<i64>,         // 生日 (不填写就是今天)
    pub description: Option<String>,   // 可选的描述
    pub desc_at: Vec<i64>,             // 描述 - 艾特的用户 IDs
    pub perm_id: Option<i16>,          // 权限
    pub lat: Option<String>,           // 纬度
    pub lng: Option<String>,           // 经度
    pub last_login_ip: Option<String>, // 最后登录IP
    pub status: Option<i16>,           // 状态：0. 失效 1. 正常
    pub register_type: Option<i16>, // 注册来源类型（1: 手机, 2: 邮箱, 3: 苹果, 4: 谷歌, 5: 微信）
    pub created_time: i64,          // 创建时间 (兼容旧版PHP)
    pub updated_time: i64,          // 更新时间 (兼容旧版PHP)
    pub created_at: DateTime<Utc>,  // 创建时间
    pub user_state: i16,            // 用户状态码
}

// 构造实现
impl UserCommand {
    //

    ////////

    /// # 0. [QUICK] - 快速构建用于注册的 Command
    /// * `desc`: 手机注册
    pub fn new_with_phone(phone_no: String) -> Self {
        let now_ts = Utc::now().timestamp();
        Self {
            phone: Some(phone_no),
            register_type: Some(1), // 默认手机注册
            created_time: now_ts,
            updated_time: now_ts,
            created_at: Utc::now(),
            ..Default::default()
        }
    }

    ////////

    /// # 0. [QUICK] - 快速构建用于注册的 Command
    /// * `desc`: 邮箱注册
    pub fn new_with_email(email: String) -> Self {
        let now_ts = Utc::now().timestamp();
        Self {
            email: Some(email),
            register_type: Some(1), // 默认手机注册
            created_time: now_ts,
            updated_time: now_ts,
            created_at: Utc::now(),
            ..Default::default()
        }
    }

    ////////

    /// # 1. [MAKE] - 注册新用户
    pub fn new(self) -> UserEntity {
        // ⏰️ 获取当前的系统秒级时间戳
        let now = Utc::now(); // UTC
        let now_ts = Utc::now().timestamp(); // UTC 时间戳

        // 兼容处理：如果命令中未传 created_time 或为 0，则用当前时间戳兜底
        let final_created_time = if self.created_time > 0 {
            self.created_time
        } else {
            now_ts
        };

        // 📱 如果客户端有传入非0的有效权限，则以客户端为准，否则默认赋予 5
        let perm = match self.perm_id {
            Some(p) if p > 0 => p,
            _ => 5,
        };

        // 🆔 处理 send_id：如果客户端不传入，服务端生成一个无分隔符的 UUID v4 取代
        let final_uuid_v4 = self
            ._id
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        //.unwrap_or_else(|| Uuid::new_v4().to_string().replace("-", ""));

        // 🆔 创建一个vx_id
        let final_vx_id = self
            .vx_id
            .unwrap_or_else(|| format!("vx_{}", Uuid::now_v7()));

        // 👤 构造默认昵称
        let reg_type = self.register_type.unwrap_or(1);
        let default_nickname = make_nickname(reg_type, self.phone.as_deref());

        // 🌐 从 last_login_ip 获取客户端 IP（网关层从 HttpRequest 提取后注入）
        // ⚠️ 兜底方案：若网关未传入 IP（或为空字符串），则标记为"未知IP"，避免 NOT NULL 冲突
        let client_ip = self
            .last_login_ip
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "未知IP".to_string());

        // 装载到User表
        UserEntity {
            _id: Option::from(final_uuid_v4), // 最终确定的 UUID 字符串
            vx_id: Option::from(final_vx_id), // 最终确定的 UUID v7 字符串
            user_type: Some(2),
            user_nickname: Some(self.nickname.unwrap_or(default_nickname)), // 客户端有传用客户端的，否则用默认生成的
            signature: Option::from(
                self.signature
                    .unwrap_or_else(|| "这个家伙很懒，这里还是空的啊~".to_string()),
            ),
            avatar: Option::from(
                self.avatar
                    .unwrap_or_else(|| "/default_avatar.png".to_string()),
            ),
            avatar_thumb: Option::from(
                self.avatar_thumb
                    .unwrap_or_else(|| "/default_avatar.png".to_string()),
            ),
            bg_img: Option::from(
                self.bg_img
                    .unwrap_or_else(|| "/default_bg_img.png".to_string()),
            ),

            birthday: Option::from(self.birthday.unwrap_or(now_ts)), // 不填写就是当前时间戳
            email: Option::from(self.email.unwrap_or_else(|| "还没有填写喔😮".to_string())),
            phone: Option::from(self.phone.unwrap_or_else(|| "还没有填写喔😮".to_string())),

            // ✅ login_ip / register_ip 使用网关层注入的客户端真实 IP
            login_ip: Some(client_ip.clone()),
            register_ip: Some(client_ip),

            status: Some(0), // 如果开启了审核，状态默认是 0
            user_status: 1,  // 用户状态码
            perm_id: perm,

            // 🕒 兼容老 PHP 系统的历史印记（i64 类型）
            create_time: final_created_time,
            created_at: Option::from(if self.created_at.timestamp() > 0 {
                self.created_at
            } else {
                now
            }),
            last_login_time: Some(final_created_time), // 👈 新用户注册时，last_login_time 用创建时间兜底，防止非空约束报错

            // 📍 坐标信息
            lat: self.lat,
            lng: self.lng,

            // 同步时间
            ..Default::default()
        }
    }
}

/// # [MAKE] - 构造一个同步 ID
/// * 格式: 时间戳_uuid
#[allow(dead_code)]
fn make_sync_id() -> String {
    let now = Utc::now();
    let uuid = Uuid::new_v4().to_string();
    format!("{}_{}", now.timestamp(), uuid)
}

/// # [MAKE] - 构造默认昵称
/// * 格式: 手机用户1044_uuid前8位
fn make_nickname(register_type: i16, phone: Option<&str>) -> String {
    // 获取注册前缀
    let prefix = UserName::from_i16(register_type).to_str();

    // 截取手机尾号 4 位
    let tail = match phone {
        Some(p) if p.len() >= 4 => &p[p.len() - 4..],
        _ => "0000",
    };

    // 获取 UUIDv4 前 8 位
    let uuid_full = Uuid::new_v4().to_string();
    let uuid_8 = &uuid_full[..8];

    format!("{}{}_{}", prefix, tail, uuid_8)
}

/// # 构造用户昵称前缀
pub enum UserName {
    Phone,
    Email,
    Apple,
    Google,
    Wechat,
    Unknown,
}

impl UserName {
    pub fn from_i16(val: i16) -> Self {
        match val {
            1 => UserName::Phone,
            2 => UserName::Email,
            3 => UserName::Apple,
            4 => UserName::Google,
            5 => UserName::Wechat,
            _ => UserName::Unknown,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UserName::Phone => "手机用户",
            UserName::Email => "邮箱用户",
            UserName::Apple => "苹果用户",
            UserName::Google => "谷歌用户",
            UserName::Wechat => "微信用户",
            UserName::Unknown => "普通用户",
        }
    }
}

//////// END
