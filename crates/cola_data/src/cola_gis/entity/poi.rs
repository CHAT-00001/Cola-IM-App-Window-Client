// data/src/cola_gis/entity/poi.rs
// 🗄 数据 - 可乐GIS - Entity - POI
// 2026/7/6 20:10 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - POI表
/// * `pg schema`: `cola_gis`
/// * `table name`: `poi`
#[derive(Debug, Clone, Default, FromRow)]
pub struct PoiEntity {
    pub id: i64,                           // 兴趣点 ID
    pub uid: i64,                          // 发布者 ID
    pub channel_id: Option<i16>,           // 频道 ID
    pub poi_type: Option<i16>,             // 地点类型
    pub title: String,                     // 标题
    pub name: String,                      // 名称
    pub name_en: Option<String>,           // 英文名称
    pub thumb: String,                     // 缩略图
    pub description: Option<String>,       // 描述
    pub thumbnail: Option<String>,         //
    pub cover_url: Option<String>,         //
    pub href: String,                      //
    pub href_w: Option<String>,            //
    pub original_url: Option<String>,      // 官府网站 URL
    pub tags: Option<Vec<String>>,         // 标签
    pub lat: Option<f64>,                  // 纬度
    pub lng: Option<f64>,                  // 经度
    pub duration: Option<String>,          // 时长
    pub width: Option<i16>,                // 宽度
    pub height: Option<i16>,               // 高度
    pub fps: Option<i16>,                  // 帧数
    pub bit: Option<i16>,                  // 色深
    pub views: i32,                        // 浏览数量
    pub likes: i32,                        // 点赞数量
    pub dislike: i32,                      // 讨厌数量
    pub collects: i32,                     // 收藏数量
    pub comments: i32,                     // 评论数量
    pub danmakus: i32,                     // 弹幕数量
    pub recommends: i32,                   // 推荐数量
    pub shares: i32,                       // 分享数量
    pub is_public: Option<bool>,           // 是否公共
    pub is_deleted: i16,                   // 是否删除
    pub status: i16,                       // 状态码
    pub music_id: Option<i64>,             // 音乐 ID
    pub goods_id: Option<i64>,             // 商品 ID
    pub visibility_perm: i16,              // 可见权限
    pub comment_perm: i16,                 // 评论权限
    pub danmaku_perm: i16,                 // 弹幕权限
    pub collect_perm: i16,                 // 收藏权限
    pub download_perm: i16,                // 下载权限
    pub add_time: i64,                     // 添加时间 - 机器 (时间戳)
    pub upd_time: Option<i64>,             // 更新时间 - 机器 (时间戳)
    pub del_time: Option<i64>,             // 删除时间 - 机器 (时间戳)
    pub sync_at: Option<i64>,              // 同步时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 - 人类
}

////////

/// # [COLUMNS] - POI - 数据表字段
/// * `desc`: `POI - 静态常量表`
pub const GIS_POI_COLUMNS: &str = r#"
    id, uid, channel_id,poi_type, title, name, name_en, description, desc_at_uids,
    thumb, thumbnail, cover_url, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, dislike, collects, comments,
    danmakus, recommends, shares, is_public, is_deleted, status,
    music_id, goods_id, visibility_perm, comment_perm, danmaku_perm, collect_perm, download_perm,
    add_time, upd_time, del_time, sync_at, created_at, updated_at, deleted_at
"#;

//////// END
