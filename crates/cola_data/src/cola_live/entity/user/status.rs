//  cola_data/src/live/entity/status.rs
// 数据 - LIVE - entity - 用户表
// 2026/8/17 17:20 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 用户状态表
/// * `pg schema`: `cola_live` -- PG 模式
/// * `table name`: `user_status`  -- 表名
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct LiveUserEntity {
    pub id: i64,                              // ID (自增 / 雪花)
    pub _id: Option<String>,                  // UUID v4
    pub user_type: Option<i16>,               // 用户类型 （默认 2 普通用户）
    pub user_nickname: Option<String>,        // 昵称
    pub signature: Option<String>,            // 签名
    pub avatar: Option<String>,               // 头像
    pub avatar_thumb: Option<String>,         // 小头像
    pub bg_img: Option<String>,               // 主页背景图
    pub email: Option<String>,                // 邮箱
    pub phone: Option<String>,                // 电话号码
    pub sns_url: Option<String>,              // 视频原始url
    pub birthday: Option<i64>,                // 生日
    pub last_login_time: Option<i64>,         // 最后登录时间(兼容旧版PHP)
    pub sex: Option<i16>,                     // 性别
    pub perm_id: i16,                         // 权限
    pub views: Option<i64>,                   // 粉丝数量
    pub likes: Option<i64>,                   // 收到的点赞数量
    pub fans: Option<i64>,                    // 粉丝数量
    pub follows: Option<i64>,                 // 关注数量
    pub level: Option<i16>,                   // 用户等级
    pub author_level: Option<i16>,            // 主播等级
    pub lat: Option<String>,                  // 纬度
    pub lng: Option<String>,                  // 经度
    pub login_ip: Option<String>,             // 当前登录IP
    pub register_ip: Option<String>,          // 注册IP
    pub status: Option<i16>,                  // 状态
    pub is_deleted: Option<bool>,             // 逻辑删除
    pub created_at: Option<DateTime<Utc>>,    // 创建时间
    pub updated_at: Option<DateTime<Utc>>,    // 更新时间
    pub deleted_at: Option<DateTime<Utc>>,    // 删除时间
    pub last_login_at: Option<DateTime<Utc>>, // 最后登录时间
    pub score: i64,                           // 积分
    pub coin: i64,                            // 钻石
    pub user_status: i16,                     // 用户状态码
}

////////

/// # 2.[COLUMNS] - 数据表原始字段（对应 Entity 的基础字段，1:1 完全一致）
pub const LIVE_USER_COLUMNS: &str = r#"
    id, _id, user_type, user_nickname, avatar, avatar_thumb, bg_img, signature, birthday,last_login_time,
    sex, perm_id, email, mobile AS phone, sns_url, more, lat, lng, country_code,
    is_ad, firstcharge_used, praise_num,
    views, likes, fans, follows, level, author_level, login_ip, register_ip,
    last_login_time, goodnum, score, votes, votestotal, province, city,
    isrecommend, openid, login_type, iszombie, isrecord, iszombiep, issuper, ishot, recommend_time, live_window,
    user_login, user_status AS status, user_pass,
    balance, balance_total, balance_consumption,
    online, online_expired_at, is_deleted, create_time, created_at, updated_at, deleted_at, last_login_at,
    score, coin, user_status
"#;

////////

/// # 2. [COLUMNS] - INSERT/RETURNING 用别名确保 COALESCE 不会在写入时报错
pub const INSERT_RETURNING: &str = r#"
    id, _id, user_type, user_nickname, avatar, bg_img,
    signature, email, phone, birthday, status, perm_id, create_time,
    login_ip, register_ip
"#;

//////// END
