// cola_data/src/cola_live/entity/room/live_room.rs  -- 数据 - LIVE - entity - 直播间实体
// 2026/7/8 10:23

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 直播间表
/// * `pg schema`: `cola_live`
/// * `table name`: `live_room`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveRoomEntity {
    pub id: i64,                           // 房间 ID
    pub _id: i64,                          // 备用 ID
    pub uid: i64,                          // 用户 ID
    pub room_id: String,                   // 直播间唯一标识 (无分隔符 UUIDv4)
    pub show_id: Option<String>,           // 秀场ID / 房间逻辑ID
    pub title: Option<String>,             // 直播标题
    pub thumb: Option<String>,             // 直播封面图URL
    pub pull: Option<String>,              // 播流/拉流地址
    pub stream: Option<String>,            // 推流名称 (唯一标识流名)
    pub channel_id: Option<i32>,           // 频道分类ID
    pub is_video: Option<i16>,             // 是否视频直播
    pub is_mic: Option<i16>,               // 是否连麦
    pub is_hot: Option<i16>,               // 是否热门
    pub is_recommend: Option<i16>,         // 是否推荐
    pub is_live: Option<i16>,              // 是否在直播
    pub is_shop: Option<i16>,              // 是否开启店铺
    pub is_off: Option<i16>,               // 是否手动关闭
    pub good_num: Option<i64>,             // 挂载商品数量
    pub anyway: Option<i16>,               // 屏幕方向：0.横屏 1.竖屏
    pub hot_votes: Option<i64>,            // 当前热度值/票数
    pub gift_total_coin: Option<i64>,      // 该场直播主播收入总金币
    pub gift_user_total: Option<i64>,      // 该场直播观众消耗总金币
    pub banker_coin: Option<i64>,          // 庄家/游戏池金币
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
    pub status: i16,                       // 状态: 0.已关闭 1.直播中 2.禁播/审核中
    pub is_deleted: Option<bool>,          // 逻辑删除
    pub add_time: i64,                     // 添加时间 - 机器
    pub upd_time: i64,                     // 更新时间 - 机器
    pub created_at: Option<DateTime<Utc>>, // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 - 人类
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间 - 人类

    // --- 扩展字段 ---
    #[sqlx(default)]
    pub distance: Option<f64>, // 距离 (SQL 实时计算得出，单位：km)
}

////////

/// # [COLUMNS] - LIVE 表
/// * `desc`: `构建live表基础字段（匹配最终版 Entity）`
pub const LIVE_ROOM_COLUMNS: &str = r#"
    uid, room_id, show_id, title, thumb, pull, stream, channel_id,
    is_video, is_mic, is_hot, is_recommend, is_live, is_shop, is_off, status,
    good_num, anyway, hot_votes, gift_total_coin, gift_user_total, banker_coin,
    pk_uid, pk_stream, video_url, province, city, address, lng, lat,
    live_type, type_val, device_info, game_action, voice_type,
    sw_player_status, sw_player_id, sw_pull_url,
    start_time, off_time, recommend_time
"#;

////////

/// # 兼容旧仓储命名
/// * `desc`: `旧 live_repo 使用 LiveEntity/LIVE_COLUMNS，统一指向当前直播间实体`
pub type LiveEntity = LiveRoomEntity;
pub const LIVE_COLUMNS: &str = LIVE_ROOM_COLUMNS;

//////// END
