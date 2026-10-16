// cola_data/src/cola_dynamic/cmd/dynamic.rs
// 数据 - ⏹ 可乐动态 - CMD - 发布命令
// 2026/6/19 17:12

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 动态创建命令
/// * `desc`: `⏹ 可乐动态 - 动态发布命令`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamicCommand {
    pub r#type: i16,
    pub uid: i64,
    pub title: String,
    pub description: Option<String>,
    pub thumb: Option<String>,
    pub video_thumb: Option<String>,
    pub href: Option<String>,
    pub voice: Option<String>,
    pub length: Option<i16>,
    pub media: MediaCommand, // 对应发布的媒体负载
    pub views: i32,
    pub likes: i32,
    pub collect: i32,
    pub comments: i32,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub view_perm: i16,
    pub comment_perm: i16,
    pub share_perm: i16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaCommand {
    pub media_type: i16,
    pub fs: String,
    pub thumbnail: String,
    pub url: String,
    pub width: i16,
    pub height: i16,
    pub fps: f32,
    pub duration: i32,
}

impl DynamicCommand {
    /// # [CASE] - 发布动态初始化
    /// 初始化 views=1，计数归零
    pub fn new(uid: i64, title: String, media: MediaCommand) -> Self {
        Self {
            r#type: 0,
            uid,
            title,
            description: None,
            thumb: None,
            video_thumb: None,
            href: None,
            voice: None,
            length: None,
            media,
            views: 1, // 按照需求初始化为1
            likes: 0,
            collect: 0,
            comments: 0,
            lat: None,
            lon: None,
            city: None,
            address: None,
            view_perm: 0,
            comment_perm: 0,
            share_perm: 0,
        }
    }
}

//////// END
