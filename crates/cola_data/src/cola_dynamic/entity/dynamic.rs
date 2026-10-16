// data/src/cola_dynamic/entity/dynamic.rs
// 🗄 数据 - ⏹ 可乐动态 - entity - 动态表
// 2026/6/19 15:12 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 动态表
///  * `pg schema`: `cola_dynamic` -- PG模式
/// * `table name`: `dynamic` -- 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DynamicEntity {
    pub id: i64,                           // ID(自增 / 雪花)
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
    pub media: MediaFs,                    // 媒体负载(新增,支持livephoto)
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
    pub isdel: i16,                        // 是否删除
    pub del_time: Option<i64>,             // 删除时间(机器)
    pub recomend: i32,                     // 推荐值
    pub view_perm: i16,                    // 浏览权限
    pub comment_perm: i16,                 // 评论权限
    pub share_perm: i16,                   // 分享权限
    pub add_time: i64,                     // 添加时间(兼容旧版PHP)
    pub upt_time: i64,                     // 更新时间(兼容旧版PHP)
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

/// # [EM] - 媒体负载
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaFs {
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
