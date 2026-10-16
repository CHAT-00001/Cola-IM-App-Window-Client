// cola_data/src/cola_im/info/chat.rs
// 🗄 数据 - ⏹ 可乐IM - info - 聊天会话
// 2026-07-07

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 聊天会话信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatInfo {
    pub id: i64,                            // ID (雪花ID)
    pub _id: String,                        // UUID v4
    pub user_id: i64,                       // 持有者
    pub title: String,                      // 标题
    pub chat_type: i16,                     // 聊天类型
    pub avatar: Option<String>,             // 头像 URL
    pub bg_img: Option<String>,             // 聊天背景图
    pub last_msg: Option<String>,           // 最后一条消息摘要
    pub last_mas_at: Option<DateTime<Utc>>, // 最后一条消息的时间
    pub is_pinned: Option<bool>,            // 是否置顶
    pub is_started: Option<bool>,           // 是否星标
    pub status: i16,                        // 状态码
    pub sort: i16,                          // 排序码
    pub add_time: i64,                      // 创建时间
    pub upd_time: i64,                      // 更新时间
}

//////// END
