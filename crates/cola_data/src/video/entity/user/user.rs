// cola_data/src/video/entity/user/user.rs  -- 数据 - USER - entity - 用户 - user
// 2026/8/1 15:06

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频 - 用户 - 用户表
/// * `pg schema`: `video`
/// * `table name`: `user`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct VideoUserEntity {
    pub id: i64, // id
    pub _id: String,
    pub user_type: Option<i16>, // 用户类型 （默认 2 普通用户）
    // -- 资料 --
    pub user_nickname: Option<String>, // 昵称
    pub signature: Option<String>,     // 签名
    pub avatar: Option<String>,        // 头像
    pub bg_img: Option<String>,        // 主页背景图
    pub user_email: Option<String>,    // 邮箱
    pub phone: Option<String>,         // 电话号码
    pub sns_url: Option<String>,       // 视频原始url
    pub birthday: Option<i64>,         // 生日
    pub sex: Option<i16>,              // 性别
    // -- 权限
    pub perm_id: i16, // 权限
    // -- 统计
    pub likes: Option<i64>,        // 收到的点赞数量
    pub fans: Option<i64>,         // 粉丝数量
    pub follows: Option<i64>,      // 关注数量
    pub level: Option<i16>,        // 用户等级
    pub author_level: Option<i16>, // 主播等级
    // -- 来源 --
    pub lat: Option<f64>,    // 纬度
    pub lng: Option<f64>,    // 经度
    pub login_ip: String,    // 当前登录IP
    pub register_ip: String, // 注册IP
    // -- 状态 --
    pub status: Option<i16>, // 状态
    // -- time --
    pub create_time: i64,                  // 创建时间（机器）
    pub created_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub updated_at: Option<DateTime<Utc>>, // 更新时间（人类）
}

////////

/// #[COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const USER_COLUMNS: &str = r#"
    id, _id, user_type, sex, snow_id, user_nickname, avatar, bg_img, signature, birthday,
    user_email, mobile, more, lat, lng, country_code,
    is_ad, firstcharge_used, praise_num
    views, likes, fans, follows,
    last_login_time,goodnum, score, votes, votestotal, province, city,
    isrecommend, openid, login_type, iszombie, isrecord, iszombiep, issuper,ishot, recommend_time,live_window
    user_login, user_status, user_pass,
    balance, balance_total, balance_consumption,
    online, online_expired_at, create_time, created_at, updated_at
"#;

//////// END
