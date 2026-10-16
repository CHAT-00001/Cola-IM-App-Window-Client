// cola_data/src/cola_dynamic/info/cola_dynamic.rs
// 🗄 数据 - ⏹ 可乐动态 - INFO - 动态信息
// 2026/6/19 16:48 Created.

////////

use crate::cola_dynamic::entity::dynamic::DynamicEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [ENTITY] - 动态信息
/// * `desc`: `⏹ 可乐动态 - 动态信息`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicInfo {
    pub id: i64,                           // ID (自增 / 雪花)
    pub _id: Option<String>,               // UUID v4
    pub r#type: i16,                       // 类型
    pub uid: i64,                          // 用户ID
    pub title: String,                     // 标题
    pub description: Option<String>,       // 描述
    pub thumb: Option<String>,             // 照片封面
    pub video_thumb: Option<String>,       // 视频封面
    pub href: Option<String>,              // 视频地址
    pub voice: Option<String>,             // 语音
    pub length: Option<i16>,               // 语音长度
    pub media: Media,                      // 媒体负载(新增,支持livephoto)
    pub views: i32,                        // 浏览数量
    pub likes: i32,                        // 点赞数量
    pub collect: i32,                      // 收藏数量
    pub comments: i32,                     // 评论数量
    pub original_url: Option<String>,      // 原始URL
    pub status: i16,                       // 状态
    pub xiajia_reason: Option<String>,     // 下架原因
    pub lat: Option<f64>,                  // 纬度
    pub lon: Option<f64>,                  // 经度
    pub city: Option<String>,              // 城市
    pub address: Option<String>,           // 地址
    pub fail_reason: Option<String>,       // 失败原因
    pub show_val: Option<i32>,             // 显示值
    pub recommend_val: Option<i32>,        // 推荐值(旧)
    pub labelid: Option<i32>,              // 主题ID
    pub goodsid: Option<i32>,              // 商品ID
    pub goods_isxiajia: Option<i16>,       // 商品是否下架
    pub isdel: i16,                        // 是否删除(旧版)
    pub recomend: i32,                     // 推荐值
    pub view_perm: i16,                    // 浏览权限
    pub comment_perm: i16,                 // 评论权限
    pub share_perm: i16,                   // 分享权限
    pub add_time: i64,                     // 添加时间(兼容旧版PHP)
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 (软删除)
}

/// # [EM] - 媒体负载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: i64,           // ID
    pub media_type: i16,   // 媒体类型
    pub fs: String,        // 文件名称
    pub thumbnail: String, // 缩略图
    pub url: String,       // 地址
    pub width: i16,        // 帧宽度
    pub height: i16,       // 帧高度
    pub fps: f32,          // 每秒帧数
    pub duration: i32,     // 时长
}

// 转换函数
// 假设这些引用已存在

impl From<DynamicEntity> for DynamicInfo {
    fn from(entity: DynamicEntity) -> Self {
        Self {
            id: entity.id,
            _id: entity._id,
            r#type: entity.r#type,
            uid: entity.uid,
            title: entity.title,
            description: entity.description,
            thumb: entity.thumb,
            video_thumb: entity.video_thumb,
            href: entity.href,
            voice: entity.voice,
            length: entity.length,
            media: Media {
                id: entity.media.id,
                media_type: entity.media.media_type,
                fs: entity.media.fs,
                thumbnail: entity.media.thumbnail,
                url: entity.media.url,
                width: entity.media.width,
                height: entity.media.height,
                fps: entity.media.fps,
                duration: entity.media.duration,
            },
            views: entity.views,
            likes: entity.likes,
            collect: entity.collect,
            comments: entity.comments,
            original_url: entity.original_url,
            status: entity.status,
            xiajia_reason: entity.xiajia_reason,
            lat: entity.lat,
            lon: entity.lon,
            city: entity.city,
            address: entity.address,
            fail_reason: entity.fail_reason,
            show_val: entity.show_val,
            recommend_val: entity.recommend_val,
            labelid: entity.labelid,
            goodsid: entity.goodsid,
            goods_isxiajia: entity.goods_isxiajia,
            isdel: entity.isdel,
            recomend: entity.recomend,
            view_perm: entity.view_perm,
            comment_perm: entity.comment_perm,
            share_perm: entity.share_perm,
            add_time: entity.add_time,
            is_deleted: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

// 构造函数
impl DynamicInfo {
    /// # [CASE] - 默认构造
    pub fn new() -> Self {
        Self {
            id: 0,
            _id: None,
            r#type: 0,
            uid: 0,
            title: String::new(),
            description: None,
            thumb: None,
            video_thumb: None,
            href: None,
            voice: None,
            length: None,
            media: Media {
                id: 0,
                media_type: 0,
                fs: String::new(),
                thumbnail: String::new(),
                url: String::new(),
                width: 0,
                height: 0,
                fps: 0.0,
                duration: 0,
            },
            views: 0,
            likes: 0,
            collect: 0,
            comments: 0,
            original_url: None,
            status: 0,
            xiajia_reason: None,
            lat: None,
            lon: None,
            city: None,
            address: None,
            fail_reason: None,
            show_val: None,
            recommend_val: None,
            labelid: None,
            goodsid: None,
            goods_isxiajia: None,
            isdel: 0,
            recomend: 0,
            view_perm: 0,
            comment_perm: 0,
            share_perm: 0,
            add_time: 0,
            is_deleted: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }

    /// # [CASE] - 空值/未找到
    pub fn empty() -> Self {
        let mut info = Self::new();
        info.title = "动态不存在".to_string();
        info
    }
}

//////// END
