// data/src/live/info/live.rs
// 数据 - LIVE - INFO - 直播信息
// 2026/5/19 18:17 by wx: cestbon10080

use crate::cola_user::info::user::UserInfo;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 直播信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LiveInfo {
    pub id: u64,                     // 直播 ID
    pub user_id: Option<i64>,        // 用户 ID
    pub actor: Option<String>,       // 主播信息/主包信息
    pub title: String,               // 直播标题
    pub name: String,                // 直播间名称
    pub name_en: Option<String>,     // 英语名称
    pub cover_url: Option<String>,   // 封面
    pub start_time: i64,             // 开始时间
    pub stream_name: Option<String>, // 直播流名称
    pub pull: Option<String>,        // 拉流地址
    pub video_url: String,           // 视频假直播（视频URL）
    pub is_voice: bool,              // 是否语音房
    pub is_video: bool,              // 是否假直播（测试使用）
    pub way: String,                 // 是否竖屏直播
    pub add_time: i64,               // 创建时间
    pub sync_time: i64,              // 同步时间（乐观锁）
}

impl LiveInfo {
    /// 【创建场景】根据传入的参数和用户信息构建全新的直播信息
    pub fn new(
        id: u64,
        user: Option<&UserInfo>,
        actor: Option<String>,
        mut title: String,
        mut name: String,
        name_en: Option<String>,
        cover_url: Option<String>,
        start_time: i64,
        stream_name: Option<String>,
        pull: Option<String>,
        video_url: String,
        is_voice: bool,
        is_video: bool,
        way: String,
    ) -> Self {
        let server_now_time = Utc::now().timestamp();

        // 获取用户的昵称，若无用户信息则兜底为 "主播"
        let nickname = user.map(|u| u.nickname.as_str()).unwrap_or("主播");

        // 1. 直播标题为空时：自动补全为 "xxxx直播啦"
        if title.trim().is_empty() {
            title = format!("{}直播啦", nickname);
        }

        // 2. 直播间名称（name）为空时：自动补全为 "xxxx的直播间"
        if name.trim().is_empty() {
            name = format!("{}的直播间", nickname);
        }

        // 3. 处理主播名称字段（actor），如果为空就用昵称兜底
        let final_actor = actor
            .filter(|a| !a.trim().is_empty())
            .or_else(|| Some(nickname.to_string()));

        Self {
            id,
            user_id: user.map(|u| u.id),
            actor: final_actor,
            title,
            name,
            name_en,
            cover_url,
            start_time,
            stream_name,
            pull,
            video_url,
            is_voice,
            is_video,
            way,
            add_time: server_now_time,
            sync_time: server_now_time,
        }
    }

    /// 【查询/浏览场景】从数据库纯净的元数据转换为前端 VO，同时应用文案兜底
    pub fn from_entity(
        id: u64,
        user_id: Option<i64>,
        nickname: Option<&str>, // 数据库联查出来的用户昵称，用于文案兜底
        actor: Option<String>,
        mut title: String,
        mut name: String,
        name_en: Option<String>,
        cover_url: Option<String>,
        start_time: i64,
        stream_name: Option<String>,
        pull: Option<String>,
        video_url: String,
        is_voice: bool,
        is_video: bool,
        way: String,
        add_time: i64,
        sync_time: i64,
    ) -> Self {
        let current_nickname = nickname.unwrap_or("主播");

        if title.trim().is_empty() {
            title = format!("{}直播啦", current_nickname);
        }

        if name.trim().is_empty() {
            name = format!("{}的直播间", current_nickname);
        }

        let final_actor = actor
            .filter(|a| !a.trim().is_empty())
            .or_else(|| Some(current_nickname.to_string()));

        Self {
            id,
            user_id,
            actor: final_actor,
            title,
            name,
            name_en,
            cover_url,
            start_time,
            stream_name,
            pull,
            video_url,
            is_voice,
            is_video,
            way,
            add_time,
            sync_time,
        }
    }

    /// 【异常兜底场景】当 Repo 未命中、直播被关闭或下架时，返回安全的空响应
    pub fn empty() -> Self {
        let now_timestamp = Utc::now().timestamp();

        Self {
            id: 0,
            user_id: None,
            actor: Some("主播不存在".to_string()),
            title: "直播已结束".to_string(),
            name: "未知直播间".to_string(),
            name_en: None,
            cover_url: None,
            start_time: now_timestamp,
            stream_name: None,
            pull: None,
            video_url: "".to_string(),
            is_voice: false,
            is_video: false,
            way: "1".to_string(), // 默认竖屏或横屏的初始代号
            add_time: now_timestamp,
            sync_time: now_timestamp,
        }
    }
}
