// utils/record.rs  -- utils -  构建直播流名称
// 2026/6/13 06:11

////////

use chrono::Utc;

////////

/// # [BUILD UTILS] - 直播推流名称构造（带时间戳升级版）
/// * `格式`: `uid_liveid_年月日时分秒_uuid前8位`
/// * `例子`: `10086_8888_20260613153022_a1b2c3d4`
pub fn build_stream_name(uid: i64, live_id: i64) -> String {
    // 1. 获取当前系统时间，转为本地时间或 UTC 格式的字符串 (年月日时分秒)
    // 提示：用当前年份 2026 跑出来的格式会是 "20260613153022" 这种纯数字
    let time_str = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // 2. 生成 UUID v4 并截取前 8 位
    let uuid_short: String = uuid::Uuid::new_v4().to_string().chars().take(8).collect();

    // 3. 完美拼接
    format!("{}_{}_{}_{}", uid, live_id, time_str, uuid_short)
}
