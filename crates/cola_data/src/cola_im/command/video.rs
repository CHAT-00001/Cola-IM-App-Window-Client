// cola_data/src/new/cmd/new.rs  -- 数据中心 - VIDEO - Command  - 发布视频
// 2026/5/19 21:28 by wx: cestbon10080

////////

use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use crate::video::entity::video::video::VideoEntity;

////////

/// # [COMMAND] - 视频发布命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoCommand {
    pub user_id: i64,                 // 用户 ID
    pub uuid: String,                 // UUID (客户端生成)
    pub title: String,                // 视频标题
    pub title_at: Vec<i64>,           // 标题 - 艾特的用户 IDs
    pub original_url: Option<String>, // 视频原始地址
    pub cover_url: String,            // 封面图地址
    pub thumb: String,                // 缩略图
    pub thumb_w: String,              // 缩略图 w
    pub href: String,                 // 视频地址
    pub description: Option<String>,  // 可选的描述
    pub desc_at: Vec<i64>,            // 描述 - 艾特的用户 IDs
    pub category_id: i16,             // 分类 ID
    pub music_id: Option<i64>,        // 音乐ID
    pub label_id: Option<i64>,        // 标签 ID
    pub tags: Vec<String>,            // 标签列表
    pub views: i64,                   // 浏览数量
    pub likes: i64,                   // 点赞数量
    pub comments: i64,                // 评论数量
    pub danmakus: i64,                // 弹幕数量
    pub collects: i64,                // 收藏数量
    pub shares: i64,                  // 分享数量
    pub width: Option<i16>,           // 视频宽
    pub height: Option<i16>,          // 视频高
    pub fps: Option<i16>,             // 帧数
    pub lat: Option<f64>,             // 纬度
    pub lng: Option<f64>,             // 经度
    pub visibility_perm: i16,   // 浏览权限
    pub comment_perm: i16,      // 评论权限
    pub danmaku_perm: i16,      // 弹幕权限
    pub collect_perm: i16,      // 收藏权限
    pub download_perm: i16,     // 下载权限
    pub sync_at: Option<i64>,         // 服务器同步时间
    pub created_at: i64,              // 客户端生成
    pub updated_at: i64,              // 更新时间
}

/// # [BUILD] - 构造函数
impl VideoCommand {
    /// 将发布的 Command 转换为视频核心领域实体 (VideoEntity)
    /// 同时注入通过安全校验的真实真实操作者 uid，以及由底层生成的唯一自增 video_id
    pub fn into_entity(self, real_uid: i64, real_video_id: i64) -> VideoEntity {
        // ⏰️ 获取当前的系统秒级时间戳
        let now_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // 📱 如果客户端有传入非0的有效权限，则以客户端为准，否则默认赋予 5
        let get_perm = |client_perm: i16| -> i16 { if client_perm > 0 { client_perm } else { 5 } };

        VideoEntity {
            id: real_video_id, // 动态注入
            uid: real_uid, // 动态注入
            music_id: self.music_id,
            title: self.title,
            description: self.description,
            thumbnail: Some(self.cover_url),
            href: self.href, // 经典视频流播放地址

            // 📊 计数控制初始化：浏览量默认为 1，其余核心计数规整为 0
            views: 1,
            likes: 0,
            comments: 0,
            danmakus: 0,
            collects: 0,
            shares: 0,
            recommends: 0,

            // 📐 基础流媒体元数据
            width: self.width,
            height: self.height,
            bit: None, // 码率由转码服务后续生成

            // 🕒 经典秒级 add_time，兼容老 PHP 系统的历史印记
            addtime: now_ts,

            // 📍 坐标信息：以客户端传入为准
            lat: self.lat,
            lng: self.lng,

            // 🔐 权限策略：优先以客户端传入为准，若无传入则兜底给默认权限值 5
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
