// data/src/video/info/video_resp -- 数据 - 可乐视频 - info - 视频信息
// 2026/5/21 08:40 Created.

////////

use crate::app::page::PageInfo;
use crate::video::entity::video::video::VideoEntity;
use crate::video::vo::video::VideoVo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 - 视频信息
/// * `desc`: `安全的视频元信息`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub id: i64,                       // 视频 ID
    pub _id: Option<String>,           // UUID v4
    pub uid: i64,                      // 用户 ID
    pub category_id: Option<i16>,      // 分类 ID
    pub channel_id: Option<i16>,       // 频道 ID
    pub title: String,                 // 标题
    pub description: Option<String>,   // 描述
    pub thumb: String,                 // 封面(旧版)
    pub thumbnail: Option<String>,     // 封面 2
    pub cover_media: Option<i64>,      // 封面媒体(新版)
    pub video_media: Option<Vec<i64>>, // 视频媒体(新版)
    pub href: String,                  // 视频url (旧版)
    pub original_url: Option<String>,  // 视频原始url
    pub is_4k: i16,                    // 是否4k
    pub tags: Option<Vec<String>>,     // 标签
    pub duration: Option<String>,      // 时长
    pub width: Option<i16>,            // 帧宽度
    pub height: Option<i16>,           // 帧高度
    pub fps: Option<i16>,              // 帧数
    pub length: Option<i32>,           // 时长(ms)
    pub views: i32,                    // 浏览量
    pub done_views: Option<i32>,       // 完成播放数量
    pub likes: i32,                    // 点赞量
    pub dislike: i32,                  // 被踩数量
    pub collects: i32,                 // 收藏量
    pub comments: i32,                 // 评论数量
    pub danmakus: i32,                 // 弹幕数量
    pub shares: i32,                   // 分享数量
    // -- perm
    pub visibility_perm: i16, // 可见权限
    pub comment_perm: i16,    // 评论权限
    pub danmaku_perm: i16,    // 弹幕权限
    pub collect_perm: i16,    // 收藏权限
    pub download_perm: i16,   // 下载权限
    // -- time
    pub add_time: i64,                     // 发布时间 (兼容旧版PHP)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
}

/// # [BUILD] - 构造视频信息
impl VideoInfo {
    //

    ////////

    /// # 1. [BUILD] - 空
    /// * `desc`: `专门用于返回“视频不存在”的空对象`
    pub fn empty() -> Self {
        Self {
            id: 0,
            _id: None,
            uid: 0,
            category_id: None,
            channel_id: None,
            title: "视频不存在".to_string(),
            description: None,
            thumb: String::new(),
            thumbnail: None,
            cover_media: None,
            video_media: None,
            href: String::new(),
            original_url: None,
            is_4k: 0,
            tags: None,
            duration: None,
            width: None,
            height: None,
            length: None,
            fps: None,
            views: 0,
            done_views: None,
            likes: 0,
            dislike: 0,
            collects: 0,
            comments: 0,
            danmakus: 0,
            shares: 0,
            visibility_perm: 0,
            comment_perm: 0,
            danmaku_perm: 0,
            collect_perm: 0,
            download_perm: 0,
            add_time: 0,
            created_at: None,
        }
    }

    ////////

    /// # 2. [FROM] - 转换
    /// * `desc`: `纯粹的从数据库实体转换`
    pub fn from_entity(entity: VideoEntity) -> Self {
        Self {
            id: entity.id,                           // 视频 ID
            _id: entity._id,                         // UUID v4
            uid: entity.uid,                         // 作者 ID
            category_id: entity.category_id,         // 分类 ID
            channel_id: entity.channel_id,           // 频道 ID
            title: entity.title,                     // 标题
            description: entity.description,         // 描述
            thumb: entity.thumb,                     // 封面 URL
            thumbnail: entity.thumbnail,             // 封面
            cover_media: entity.cover_media,         // 封面媒体
            video_media: entity.video_media,         // 视频媒体
            href: entity.href,                       // 视频链接
            original_url: entity.original_url,       // 原始 URL
            is_4k: entity.is_4k,                     // 是否4K
            tags: entity.tags,                       // 标签
            duration: entity.duration,               // 时长
            width: entity.width,                     // 帧宽度
            height: entity.height,                   // 帧高度
            fps: entity.fps,                         // 帧数
            length: entity.length,                   // 时长(ms)
            views: entity.views,                     // 浏览数量
            done_views: entity.done_views,           // 完播数量
            likes: entity.likes,                     // 点赞数量
            dislike: entity.dislike,                 // 不喜欢数量
            collects: entity.collects,               // 收藏数量
            comments: entity.comments,               // 评论数量
            danmakus: entity.danmakus,               // 弹幕数量
            shares: entity.shares,                   // 分享数量
            visibility_perm: entity.visibility_perm, // 可见权限
            comment_perm: entity.comment_perm,       // 评论权限
            danmaku_perm: entity.danmaku_perm,       // 弹幕权限
            collect_perm: entity.collect_perm,       // 收藏权限
            download_perm: entity.download_perm,     // 下载权限
            add_time: entity.addtime,                // 创建时间 (兼容旧版PHP)
            created_at: entity.created_at,           // 创建时间 (新版)
        }
    }
}

////////

/// # [RESPONSE] - 单视频响应
#[derive(Debug, Serialize)]
pub struct VideoSingleResponse {
    pub info: VideoVo, // 吐给前端完美的、组装好的 VO 列表
}

////////

/// # [RESPONSE] - 多视频响应
#[derive(Debug, Serialize)]
pub struct VideoListResponse {
    pub list: Vec<VideoVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 构造实现
impl VideoListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for VideoListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
