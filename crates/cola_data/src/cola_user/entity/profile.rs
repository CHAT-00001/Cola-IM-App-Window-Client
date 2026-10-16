// cola_data/src/user/entity/profile.rs -- 数据 - USER - entity - 资料表
// 2026/8/3 20:07 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 资料表
/// * `pg schema`: `cola_user`
/// * `table name`: `profile`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserProfileEntity {
    pub id: i64,                           // ID (自增 / 雪花 )
    pub uuid: Option<String>,              // uuid
    pub user_type: Option<i16>,            // 用户类型 （默认 2 普通用户）
    pub user_nickname: Option<String>,     // 昵称
    pub signature: Option<String>,         // 签名
    pub avatar: Option<String>,            // 头像
    pub bg_img: Option<String>,            // 主页背景图
    pub email: Option<String>,             // 邮箱
    pub phone: Option<String>,             // 电话号码
    pub sns_url: Option<String>,           // 视频原始url
    pub birthday: Option<i64>,             // 生日
    pub sex: Option<i16>,                  // 性别
    pub perm_id: i16,                      // 权限
    pub likes: Option<i64>,                // 收到的点赞数量
    pub fans: Option<i64>,                 // 粉丝数量
    pub follows: Option<i64>,              // 关注数量
    pub level: Option<i16>,                // 用户等级
    pub author_level: Option<i16>,         // 主播等级
    pub lat: Option<f64>,                  // 纬度
    pub lng: Option<f64>,                  // 经度
    pub login_ip: String,                  // 当前登录IP
    pub register_ip: String,               // 注册IP
    pub status: Option<i16>,               // 状态码: 0失效 1有效
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub create_time: i64,                  // 创建时间（兼容PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
/// * `说明`: 给sqlx使用
pub const USER_PROFILE_COLUMNS: &str = r#"
    id, uuid, user_type, sex, snow_id, user_nickname, avatar, bg_img, signature, birthday,
    email, mobile, more, lat, lng, country_code,
    is_ad, firstcharge_used, praise_num
    views, likes, fans, follows,
    last_login_time,goodnum, score, votes, votestotal, province, city,
    isrecommend, openid, login_type, iszombie, isrecord, iszombiep, issuper,ishot, recommend_time,live_window
    user_login, user_status, user_pass,
    balance, balance_total, balance_consumption,
    online, online_expired_at, status, is_deleted,
    create_time, created_at, updated_at, deleted_at
"#;

//////// END
