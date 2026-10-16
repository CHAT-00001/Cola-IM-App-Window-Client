// cola_data/src/code/entity/node.rs
// 数据 - AUTH - entity - 设备
// 2026/5/26 07:40

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 认证中心 - 用户设备信任及多设备状态表
/// * `pg schema`: `code` - PG 模式
/// * `table name`: `auth_device` - 表名
/// * 业务场景：支持同平台多设备登录、单设备踢出、设备锁管理、多端推送（如绑定个推/APNs的 Token）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AuthDeviceEntity {
    pub id: i64,                                     // 主键 ID (PG 大自增)
    pub user_id: i64,                                // 用户 ID
    pub device_sn: String, // 设备硬件唯一序列号 (如 iOS 的 IDFV, Android 的 ANDROID_ID, 网页生成的固定 UUID)
    pub platform: i32,     // 平台类型: 1-iOS, 2-Android, 3-Web, 4-Windows, 5-Mac
    pub device_name: String, // 设备名称 (如: "iPhone 15 Pro", "小米14 Ultra")
    pub os_version: String, // 操作系统版本 (如: "iOS 17.4", "Android 14")
    pub app_version: String, // 客户端 App 版本号 (如: "1.0.4")
    pub access_token: String, // 当前设备持有的访问令牌 Token
    pub refresh_token: String, // 当前设备持有的刷新 Token
    pub last_ip: String,   // 最近一次连接的 IP 地址 (兼容 IPv4/IPv6)
    pub is_online: i16,    // 是否在线: 1-在线, 0-离线 (用于短视频/直播网关做长连接/状态判定)
    pub status: i16,       // 设备授权状态: 1-正常, 0-已注销, -1-被挤下线, -2-已被管理员强踢/禁用
    pub expired_time: i64, // 凭证过期时间戳 (i64 杜绝2038年时间炸弹)
    pub last_active_at: i64, // 最近一次活跃/心跳时间戳
    pub created_time: chrono::DateTime<chrono::Utc>, // 设备初次登录/绑定时间
    pub updated_time: chrono::DateTime<chrono::Utc>, // 信息最后同步更新时间
}

////////

/// # [COLUMNS] SQLx 静态常量表
/// # 1. 统一的设备查询字段 (1:1 严格对齐结构体，干净、便于 SQLx 查询复用)
pub const DEVICE_COLUMNS: &str = r#"
    id, user_id, device_sn, platform, device_name, os_version, app_version,
    access_token, refresh_token, last_ip, is_online, status,
    expired_time, last_active_at, created_time, updated_time
"#;
//////// END
