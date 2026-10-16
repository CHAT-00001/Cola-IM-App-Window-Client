// cola_data/src/live/entity/live_setting.rs  -- 数据中心 - LIVE - entity - 直播间设置
// 2026/7/8 10:52

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播间 设置 实体
/// * `pg schema`: `cola_live` - PG 模式
/// * `table name`: `room_setting` - 表名
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveSettingEntity {
    pub id: i64,                           // 房间 ID
    pub _id: i64,                          // 备用 ID
    pub uid: i64,                          // 用户 ID
    pub room_id: String,                   // 直播间唯一标识 (无分隔符 UUIDv4)
    pub is_video: Option<i16>,             // 是否视频直播
    pub is_mic: Option<i16>,               // 是否连麦
    pub is_hot: Option<i16>,               // 是否热门
    pub is_recommend: Option<i16>,         // 是否推荐
    pub is_live: Option<i16>,              // 是否在直播
    pub is_shop: Option<i16>,              // 是否开启店铺
    pub is_off: Option<i16>,               // 是否手动关闭
    pub status: i16,                       // 状态: 0.已关闭 1.直播中 2.禁播/审核中
    pub pk_uid: Option<i64>,               // 正在PK的对象UID
    pub pk_stream: Option<String>,         // 正在PK的对方流名
    pub video_url: Option<String>,         // 回放视频地址
    pub province: Option<String>,          // 省份
    pub city: Option<String>,              // 城市
    pub address: Option<String>,           // 详细地理位置
    pub lng: Option<f64>,                  // 经度
    pub lat: Option<f64>,                  // 纬度
    pub live_type: Option<String>,         // 房间类型: 0.普通 1.付费 2.密码 3.门票
    pub type_val: Option<String>,          // 类型对应值: 如密码内容或门票价格
    pub device_info: Option<String>,       // 开播设备信息 (ios/android)
    pub game_action: Option<String>,       // 当前游戏状态/动作
    pub voice_type: Option<String>,        // 语音房细分类型
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub del_time: Option<i64>,             // 删除时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 - 人类
    #[sqlx(default)]
    pub distance: Option<f64>, // 距离 (SQL 实时计算得出，单位：km)
}

////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const LIVE_ROOM_COLUMNS: &str = r#"
    id, _id, uid, room_id, show_id, title, thumb, pull, stream, channel_id,
    is_video, is_mic, is_hot, is_recommend, is_live, is_shop, is_off, status,
    good_num,anyway,hot_votes,gift_total_coin, gift_user_total, banker_coin,
    add_time, upd_time, created_at, updated_at
"#;

//////// END
