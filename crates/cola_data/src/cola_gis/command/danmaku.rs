// cola_data/src/cola_gis/cmd/danmaku.rs  -- 数据中心 - GIS - cmd - 弹幕
// 2026/5/19 22:13 by wx: cestbon10080

////////

use crate::cola_gis::entity::danmaku::PoiDanmakuEntity;
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 兴趣点 弹幕 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `poi_danmaku`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiDanmakuCommand {
    pub user_id: i64,          // 用户 ID
    pub poi_id: i64,           // 兴趣点 ID
    pub danmaku_type: i16,     // 弹幕类型: 0. 固定 1. 滚动 2. ..
    pub channel_id: i16,       // 通道 ID
    pub content: String,       // 内容
    pub play_time: i32,        // 播放器时间
    pub duration: i16,         // 时长
    pub color: Option<String>, // 弹幕颜色
    pub mode: i16,             // 模式
    pub add_time: i64,         // 添加时间 - 机器
    pub visibility: i16,       // 可见性范围
    pub status: i16,           // 状态码 0. 失效 1. 正常
}

/// # [BUILD] - 构造
impl PoiDanmakuCommand {
    //

    ////////

    /// # [FROM] - 实体转换
    pub fn into_entity(self, real_uid: i64, real_poi_id: i64) -> PoiDanmakuEntity {
        // 本机时间
        let now_ts = chrono::Local::now().timestamp();

        // 默认停留时间 5000 
        let final_duration = if self.duration == 0 {
            5000
        } else {
            self.duration
        };

        // 弹幕 color 默认颜色"#FFFFFF"
        let final_color = match self.color {
            Some(ref c) if !c.is_empty() => self.color,
            _ => Some("#FFFFFF".to_string()),
        };

        // 实体
        PoiDanmakuEntity {
            user_id: real_uid,           // 用户ID
            poi_id: real_poi_id,         // 兴趣点 ID
            channel_id: self.channel_id, // 通道 1
            content: self.content,       // 内容
            play_time: self.play_time,   // 播放器时间
            duration: final_duration,    // 停留时间 5000ms
            color: final_color,          // 弹幕颜色 (默认"#FFFFFF")
            mode: self.mode,             // 模式
            likes: 0,                    // 点赞数量
            dislikes: 0,                 // 讨厌数量
            visibility: 5,               // 可见性范围
            status: self.status,         // 状态
            sync_time: now_ts,           // 同步时间
            ..Default::default()
        }
    }
}

//////// END
