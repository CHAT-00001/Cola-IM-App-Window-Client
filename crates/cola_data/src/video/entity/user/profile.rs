// cola_data/src/new/entity/user/profile.rs  --
// 数据 - VIDEO - Entity - user - 创作者表
// 2026/5/22 18:44

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频 - 用户资料
/// * table name: video_user_perm
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoUserPermEntity {
    pub id: i64,                 // 用户 ID
    pub send_id: Option<String>, // 发送 ID （客户端构造uuid v4 无分隔符）
    pub sync_id: Option<String>, // 同步 ID （服务端生成，uuid v4 无分隔符）
    pub user_type: Option<i16>,  // 用户类型 （默认 2 普通用户）
    // -- 资料 --
    pub user_nickname: Option<String>, // 昵称
    pub signature: Option<String>,     // 签名
    pub avatar: Option<String>,        // 头像
    pub bg_img: Option<String>,        // 主页背景图
    pub email: Option<String>,         // 邮箱
    pub phone: Option<String>,         // 电话号码
    pub user_url: Option<String>,      // 视频原始url
    pub birthday: Option<i64>,         // 生日
    pub sex: Option<i16>,              // 性别
    // -- 权限
    pub perm_id: i16, // 权限
    // -- 权限
    pub view_perm: i16,     // 浏览权限
    pub publish_perm: i16,  // 发布权限
    pub like_perm: i16,     // 点赞权限
    pub collect_perm: i16,  // 收藏权限
    pub download_perm: i16, // 下载权限
    pub comment_perm: i16,  // 评论权限
    pub danmaku_perm: i16,  // 弹幕权限

    pub level: Option<i16>,        // 用户等级
    pub author_level: Option<i16>, // 主播等级
    // -- 来源 --
    pub lat: Option<f64>,              // 纬度
    pub lng: Option<f64>,              // 经度
    pub login_ip: std::net::IpAddr,    // 当前登录IP
    pub register_ip: std::net::IpAddr, // 注册IP
    // -- 状态 --
    pub status: Option<i16>, // 状态
    // -- time --
    pub add_time: i32,            // 创建时间（兼容PHP程序）
    pub sync_time: Option<i64>,   // 同步时间
    pub create_time: Option<i32>, // 创建时间戳（UTC）
    pub update_time: Option<i32>, // 修改时间 （UTC）
}
