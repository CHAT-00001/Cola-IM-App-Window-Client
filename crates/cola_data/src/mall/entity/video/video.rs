// video/entity/video/info
// 数据 - 视频 - entity - video - 视频表
// 2026/5/19 21:34 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

////////

/// # [ENTITY] - ▶ 可乐视频 视频表
/// * `pg schema`: `video`
/// * `table name`: `video`
#[derive(Debug, Clone, Default, FromRow)]
pub struct VideoEntity {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub _sn: Option<i64>,                  // 雪花 ID
    pub _a: Option<String>,                // 识别码
    pub uid: i64,                          // 用户 ID
    pub category_id: Option<i16>,          // 分类 ID
    pub channel_id: Option<i16>,           // 频道 ID
    pub title: String,                     // 标题
    pub title_at_uids: Option<Vec<i64>>,   // 标题@的IDs
    pub description: Option<String>,       // 描述
    pub desc_at_uids: Option<Vec<i64>>,    // 标题@的IDs
    pub thumb: String,                     // 封面(旧版)
    pub thumb_s: Option<String>,           // 水印封面(旧版)
    pub thumbnail: Option<String>,         // 封面 2
    pub cover_media: Option<i64>,          // 封面媒体(新版)
    pub video_media: Option<Vec<i64>>,     // 视频地址(新版)
    pub href: String,                      // 视频url (旧版)
    pub href_w: Option<String>,            // 视频url w (旧版)
    pub original_url: Option<String>,      // 视频原始url
    pub is_4k: i16,                        // 是否4k
    pub tags: Option<Vec<String>>,         // 标签
    pub lat: Option<f64>,                  // 纬度
    pub lng: Option<f64>,                  // 经度
    pub duration: Option<String>,          // 时长
    pub width: Option<i16>,                // 帧宽度
    pub height: Option<i16>,               // 帧高度
    pub fps: Option<i16>,                  // 帧数
    pub length: Option<i32>,               // 时长(ms)
    pub bit: Option<i16>,                  // 色深
    pub views: i32,                        // 浏览量
    pub done_views: Option<i32>,           // 完成播放数量
    pub likes: i32,                        // 点赞量
    pub dislike: i32,                      // 被踩数量
    pub collects: i32,                     // 收藏量
    pub comments: i32,                     // 评论数量
    pub danmakus: i32,                     // 弹幕数量
    pub steps: i32,                        // 踩数量
    pub recommends: i32,                   // 推荐数量
    pub shares: i32,                       // 分享数量
    pub is_public: Option<bool>,           // 是否公开
    pub is_del: i16,                       // 是否删除 (兼容旧版PHP)
    pub status: i16,                       // 状态码: 0.下架 1.正常 2.草稿 3.冻结
    pub music_id: Option<i64>,             // 音乐id
    pub goods_id: Option<i64>,             // 商品id
    pub visibility_perm: i16,              // 可见权限
    pub comment_perm: i16,                 // 评论权限
    pub danmaku_perm: i16,                 // 弹幕权限
    pub collect_perm: i16,                 // 收藏权限
    pub download_perm: i16,                // 下载权限
    pub addtime: i64,                      // 创建时间（兼容旧版PHP）
    pub sync_at: Option<i64>,              // 同步时间
    pub is_deleted: Option<bool>,          // 是否删除
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_COLUMNS: &str = r#"
    id, _id, _sn, _a, uid, category_id, channel_id,
    title, title_at_uids, description, desc_at_uids,
    thumb, thumb_s, thumbnail, cover_media, video_media, href, href_w, original_url, is_4k, tags, lat, lng,
    duration, width, height, fps, length, bit,
    views, done_views, likes, dislike, collects, comments, danmakus, steps, recommends, shares,
    is_public, is_del, status,
    music_id, goods_id,
    visibility_perm, comment_perm, danmaku_perm, collect_perm, download_perm,
    addtime, sync_at, is_deleted, created_at, updated_at, deleted_at
"#;

//////// END
