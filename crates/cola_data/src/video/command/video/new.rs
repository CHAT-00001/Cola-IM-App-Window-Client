// cola_data/src/video/cmd/video/new.rs -- DATA - VIDEO - cmd - video - 发布视频
// 2026/5/19 21:28

////////

use crate::video::entity::video::video::VideoEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 视频发布命令
/// * `desc`: 结构体级 `#[serde(default)]`，客户端任何字段缺失都用默认值
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoNewCommand {
    pub _id: Option<String>,          // UUID (客户端生成)
    pub _sn: Option<i64>,             // 雪花ID
    pub uid: i64,                     // 用户 ID（服务端注入）
    pub title: String,                // 视频标题
    pub title_at: Vec<i64>,           // 标题 - 艾特的用户 IDs
    pub cover_url: String,            // 封面图地址
    pub thumb: String,                // 缩略图
    pub thumb_w: String,              // 缩略图 w
    pub original_url: Option<String>, // 视频原始地址
    pub is_4k: i16,                   // 是否4k
    pub href: String,                 // 视频地址
    pub description: Option<String>,  // 可选的描述
    pub desc_at: Vec<i64>,            // 描述 - 艾特的用户 IDs
    pub category_id: i16,             // 分类 ID
    pub music_id: Option<i64>,        // 音乐ID
    pub label_id: Option<i64>,        // 标签 ID
    pub tags: Vec<String>,            // 标签列表
    pub views: i32,                   // 浏览数量
    pub done_views: i32,              // 完播数量
    pub likes: i32,                   // 点赞数量
    pub dislikes: i32,                // 不喜欢数量
    pub comments: i32,                // 评论数量
    pub danmakus: i32,                // 弹幕数量
    pub steps: i32,                   // 弹幕数量
    pub collects: i32,                // 收藏数量
    pub shares: i32,                  // 分享数量
    pub width: Option<i16>,           // 视频宽
    pub height: Option<i16>,          // 视频高
    pub fps: Option<i16>,             // 帧数
    pub lat: Option<f64>,             // 纬度
    pub lng: Option<f64>,             // 经度
    pub visibility_perm: i16,         // 浏览权限
    pub comment_perm: i16,            // 评论权限
    pub danmaku_perm: i16,            // 弹幕权限
    pub collect_perm: i16,            // 收藏权限
    pub download_perm: i16,           // 下载权限
    pub sync_at: Option<i64>,         // 服务器同步时间
}

/// # [BUILD] - 构造函数
impl VideoNewCommand {
    // 💡

    ////////

    /// # [BUILD] - 构建新视频对象
    /// 同时注入通过安全校验的真实操作者 uid，以及由底层生成的唯一雪花 ID (`real_sn`)
    pub fn into_entity(self, real_uid: i64, real_sn: i64) -> VideoEntity {
        // ⏰️ 获取当前的系统秒级时间戳
        let now_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let now = Utc::now();

        // 📱 如果客户端有传入非0的有效权限，则以客户端为准，否则默认赋予 5
        let get_perm = |client_perm: i16| -> i16 { if client_perm > 0 { client_perm } else { 5 } };

        VideoEntity {
            // 💡 id 由数据库自增接管，不在此处显式指定（或利用 ..Default::default() 留空由 DB 生成）
            _sn: Some(real_sn),              // 👈 注入雪花ID到 _sn 字段
            uid: real_uid,                   // 动态注入
            music_id: self.music_id,         // 音乐ID
            title: self.title,               // 标题
            description: self.description,   // 描述
            thumbnail: Some(self.cover_url), // 缩略图(新)
            thumb: self.thumb,               // 缩略图(旧)
            href: self.href,                 // 视频链接(旧)
            is_4k: 0,

            // 📊 计数控制初始化：浏览量默认为 1，其余核心计数规整为 0
            views: 1,
            done_views: Some(0),
            likes: 0,
            dislike: 0,
            comments: 0,
            danmakus: 0,
            steps: 0,
            collects: 0,
            shares: 0,
            recommends: 0,

            // 📐 基础流媒体元数据
            width: self.width,
            height: self.height,
            bit: None,

            // 🕒 经典秒级 add_time
            addtime: now_ts,

            // 📍 坐标信息：以客户端传入为准
            lat: self.lat,
            lng: self.lng,

            // 🔐 权限策略
            visibility_perm: get_perm(self.visibility_perm),
            comment_perm: get_perm(self.comment_perm),
            danmaku_perm: get_perm(self.danmaku_perm),
            collect_perm: get_perm(self.collect_perm),
            download_perm: get_perm(self.download_perm),

            // 🖥️
            sync_at: self.sync_at,
            ..Default::default()
        }
    }
}

//////// END
