// handler/provider.rs - 推流厂商 实体映射
// 2026/6/13 06:50

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 推流厂商 实体
/// * `table_name` - `live_stream_provider`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamProviderEntity {
    pub id: i32,                   // 数据库自增 ID
    pub provider_code: String,     // 厂商唯一标识: tencent, qiniu, wangsu, aliyun 等 (唯一索引)
    pub provider_name: String,     // 厂商中文名称
    pub api_config: String,        // 动态API配置, 存储 Key/Secret/Region 等 JSON 字符串
    pub stream_config: String,     // 推流域名、鉴权 Key 等配置 JSON 字符串
    pub sort: i16,                 // 排序权重 0-9999
    pub add_time: i32,             // 添加时间 - 机器秒级时间戳 (兼容老系统)
    pub update_time: i32,          // 更新时间 - 机器秒级时间戳
    pub created_at: DateTime<Utc>, // 创建时间 - 人类可读
    pub updated_at: DateTime<Utc>, // 更新时间 - 人类可读
    pub push_domain: String,       // 主推流域名
    pub play_flv_domain: String,   // FLV拉流域名
    pub play_hls_domain: String,   // HLS拉流域名
    pub app_name: String,          // 直播 App 名 (如: Cola_live)
    pub stream_secret: String,     // 直播流基本鉴权密钥
    pub push_url_template: String, // 推流地址模板
    pub play_flv_template: String, // FLV拉流模板
    pub play_hls_template: String, // HLS拉流模板
    pub sdk_start_cmd: String,     // SDK启动推流命令模板
    pub sdk_stop_cmd: String,      // SDK停止推流命令模板
    pub expire_seconds: i32,       // 流地址有效期(秒)
    pub remark: String,            // 备注说明
    pub status: i16,               // 状态: 0. 不启用 1. 启用
}
