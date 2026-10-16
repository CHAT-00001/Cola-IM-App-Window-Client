// cola_date/src/cola_im/info/profile.rs  -- IM - info - 联系人信息
// 2026/5/21 by wx: cestbon10080

////////

use crate::video::entity::video::video::VideoEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [MATE INFO] - 联系人资料
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileInfo {
    pub id: i64, // 视频 ID
    // -- by id
    pub uid: i64, // 用户
    //pub category_id: i16, // 分类
    pub channel_id: i16, // 频道
    // -- info
    pub title: String, // 标题
    pub thumb: String, // 缩略图地址
    pub href: String,  // 视频地址
    // -- count
    pub views: i32,    // 浏览
    pub likes: i32,    // 点赞
    pub collects: i32, // 收藏
    pub shares: i32,   // 分享
    // -- perm
    pub visibility_perm: i16, // 浏览权限
    pub comment_perm: i16,    // 评论权限
    pub danmaku_perm: i16,    // 弹幕权限
    pub download_perm: i16,   // 弹幕权限
    // -- time
    pub add_time: i64, // 发布时间
}

/// # [BUILD] - 构造视频信息
impl ProfileInfo {
    /// 1. 专门用于返回“视频不存在”的空对象
    pub fn empty() -> Self {
        Self {
            id: 0,
            title: "视频不存在".to_string(),
            uid: 0,
            channel_id: 0,
            thumb: String::new(),
            href: String::new(),
            views: 0,
            likes: 0,
            collects: 0,
            shares: 0,
            visibility_perm: 0,
            comment_perm: 0,
            danmaku_perm: 0,
            download_perm: 0,
            add_time: 0, // 或者使用 chrono::Utc::now().timestamp()
        }
    }

    /// 2. 纯粹的从数据库实体转换
    pub fn from_entity(entity: VideoEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            channel_id: entity.channel_id.unwrap_or_default(),
            title: entity.title,
            thumb: entity.thumb,
            href: entity.href,
            views: entity.views,
            likes: entity.likes,
            collects: entity.collects,
            shares: entity.shares,
            add_time: entity.addtime,
            visibility_perm: entity.visibility_perm,
            comment_perm: entity.comment_perm,
            danmaku_perm: entity.danmaku_perm,
            download_perm: entity.download_perm,
        }
    }
}

//////// END
