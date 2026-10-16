// cola_dynamic/provider  - cola_dynamic 推流厂商创建命令
// 2026/6/13 04:50

////////

use crate::cola_live::entity::provider::LiveStreamProviderEntity;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
////////

// 直播配置 - 2026/06/13 04:04

// 腾讯云
pub mod tencent {
    pub const LIVE_DOMAIN_PUSH: &str = "push.tencent.cola_live.myqcloud.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.tencent.cola_live.myqcloud.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.tencent.cola_live.myqcloud.com";
    pub const APP_NAME: &str = "Cola_live";
    pub const LIVE_SECRET_KEY: &str = "tencent_live_secret_key";
}

// 七牛
pub mod qiniu {
    pub const LIVE_DOMAIN_PUSH: &str = "push.qiniu.cola_live.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.qiniu.cola_live.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.qiniu.cola_live.com";
    pub const APP_NAME: &str = "Cola_live";
    pub const LIVE_SECRET_KEY: &str = "qiniu_live_secret_key";
}

// 网宿
pub mod wangsu {
    pub const LIVE_DOMAIN_PUSH: &str = "push.wangsu.cola_live.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.wangsu.cola_live.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.wangsu.cola_live.com";
    pub const APP_NAME: &str = "Cola_live";
    pub const LIVE_SECRET_KEY: &str = "wangsu_live_secret_key";
}

// 阿里
pub mod alicloud {
    pub const LIVE_DOMAIN_PUSH: &str = "push.aliyun.cola_live.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.aliyun.cola_live.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.aliyun.cola_live.com";
    pub const APP_NAME: &str = "Cola_live";
    pub const LIVE_SECRET_KEY: &str = "aliyun_live_secret_key";
}

// 亚马逊
pub mod amazon {
    pub const LIVE_DOMAIN_PUSH: &str = "push.aws.cola_live.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.aws.cola_live.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.aws.cola_live.com";
    pub const APP_NAME: &str = "cola_live";
    pub const LIVE_SECRET_KEY: &str = "aws_live_secret_key";
}

// 谷歌
pub mod google {
    pub const LIVE_DOMAIN_PUSH: &str = "push.google.cola_live.com";
    pub const LIVE_DOMAIN_FLV: &str = "play.google.cola_live.com";
    pub const LIVE_DOMAIN_HLS: &str = "hls.google.cola_live.com";
    pub const APP_NAME: &str = "Cola_live";
    pub const LIVE_SECRET_KEY: &str = "google_live_secret_key";
}

////////

/// # [COMMAND] - 直播流厂商 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveStreamProviderCommand {
    pub id: i32,                   // ID
    pub provider_code: String,     // 厂商唯一标识: tencent, qiniu, wangsu, aws, google, aliyun
    pub provider_name: String,     // 厂商中文名称
    pub api_config: String,        // 动态API配置, 存储Key/Secret/Region等
    pub stream_config: String,     // 推流域名、鉴权Key等配置
    pub sort: i16,                 // 排序 0-9999
    pub add_time: i32,             // 添加时间 - 机器
    pub update_time: i32,          // 更新时间 - 机器
    pub created_at: DateTime<Utc>, // 创建于 - 人类
    pub updated_at: DateTime<Utc>, // 更新于 - 人类
    pub push_domain: String,       // 主推流域名
    pub play_flv_domain: String,   // FLV拉流域名
    pub play_hls_domain: String,   // HLS拉流域名
    pub app_name: String,          // 直播App名
    pub stream_secret: String,     // 直播流鉴权密钥
    pub push_url_template: String, // 推流地址模板
    pub play_flv_template: String, // FLV拉流模板
    pub play_hls_template: String, // HLS拉流模板
    pub sdk_start_cmd: String,     // SDK启动推流命令模板
    pub sdk_stop_cmd: String,      // SDK停止推流命令模板
    pub expire_seconds: i32,       // 流地址有效期(秒)
    pub remark: String,            // 备注说明
    pub status: i16,               // 状态: 0. 不启用 1. 启用
}

// 角色枚举：用于做权限隔离，强制约束只有 Ops（运维）能调用
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Operator, // 运维人员
    Admin,
}

/// # [BUILD] - 构造函数与命令行为
impl LiveStreamProviderCommand {
    /// 运维人员专用：根据厂商 app 初始化不同的直播流配置
    /// 默认状态为 0 (不启用)，后续由开播命令激活
    pub fn new(
        role: &UserRole,       // 强制校验角色
        provider_code: String, // 厂商标识，如 "tencent"、"qiniu"
        provider_name: String, // 厂商中文名，如 "腾讯云直播"
        api_config: String,    // API 动态配置
        stream_config: String, // 推流域名等配置
    ) -> Result<Self, String> {
        // 🔒 权限硬校验：如果不是运维人员，直接拒绝创建
        if role != &UserRole::Operator {
            return Err("权限不足：该操作仅限运维人员执行".to_string());
        }

        // 🗺️ 根据传入的 provider_code 映射不同厂商的初始配置
        let (push, flv, hls, app, secret) = match provider_code.to_lowercase().as_str() {
            "tencent" => (
                tencent::LIVE_DOMAIN_PUSH,
                tencent::LIVE_DOMAIN_FLV,
                tencent::LIVE_DOMAIN_HLS,
                tencent::APP_NAME,
                tencent::LIVE_SECRET_KEY,
            ),
            "qiniu" => (
                qiniu::LIVE_DOMAIN_PUSH,
                qiniu::LIVE_DOMAIN_FLV,
                qiniu::LIVE_DOMAIN_HLS,
                qiniu::APP_NAME,
                qiniu::LIVE_SECRET_KEY,
            ),
            "wangsu" => (
                wangsu::LIVE_DOMAIN_PUSH,
                wangsu::LIVE_DOMAIN_FLV,
                wangsu::LIVE_DOMAIN_HLS,
                wangsu::APP_NAME,
                wangsu::LIVE_SECRET_KEY,
            ),
            "aliyun" | "alicloud" => (
                alicloud::LIVE_DOMAIN_PUSH,
                alicloud::LIVE_DOMAIN_FLV,
                alicloud::LIVE_DOMAIN_HLS,
                alicloud::APP_NAME,
                alicloud::LIVE_SECRET_KEY,
            ),
            "aws" | "amazon" => (
                amazon::LIVE_DOMAIN_PUSH,
                amazon::LIVE_DOMAIN_FLV,
                amazon::LIVE_DOMAIN_HLS,
                amazon::APP_NAME,
                amazon::LIVE_SECRET_KEY,
            ),
            "google" => (
                google::LIVE_DOMAIN_PUSH,
                google::LIVE_DOMAIN_FLV,
                google::LIVE_DOMAIN_HLS,
                google::APP_NAME,
                google::LIVE_SECRET_KEY,
            ),
            _ => return Err(format!("未识别的直播厂商标识: {}", provider_code)),
        };

        let now = Utc::now();
        let now_ts = now.timestamp() as i32;

        Ok(Self {
            id: 0, // 数据库自增ID，初始化给 0
            provider_code,
            provider_name,
            api_config,
            stream_config,
            sort: 0,
            add_time: now_ts,
            update_time: now_ts,
            created_at: now,
            updated_at: now,

            // 自动加载匹配到的厂商域名配置
            push_domain: push.to_string(),
            play_flv_domain: flv.to_string(),
            play_hls_domain: hls.to_string(),
            app_name: app.to_string(),

            // 初始时使用系统默认分配的厂商密钥
            stream_secret: secret.to_string(),

            // 占位模板，留空等待后续填充或开播动态生成
            push_url_template: "这里还是空的呀~!".to_string(),
            play_flv_template: "这里还是空的呀~!".to_string(),
            play_hls_template: "这里还是空的呀~!".to_string(),
            sdk_start_cmd: "这里还是空的呀~!".to_string(),
            sdk_stop_cmd: "这里还是空的呀~!".to_string(),

            expire_seconds: 86400, // 默认流客户端有效期（24小时）
            remark: "运维初始创建".to_string(),

            // 📌 初始状态严格为 0（不启用），等待开播命令将其唤醒
            status: 0,
        })
    }

    /// 由开播命令调用：将记录的状态变更为 1 (启用)，并更新动态密钥
    pub fn enable_by_live_cmd(&mut self, live_secret: String) {
        let now = Utc::now();

        self.status = 1; // 1 = 启用状态
        self.stream_secret = live_secret; // 注入开播命令生成的实时鉴权密钥
        self.update_time = now.timestamp() as i32; // 更新机器时间戳
        self.updated_at = now; // 更新人类可读时间
        self.remark = format!(
            "{} | [开播通知] 于 {} 被开播命令激活启用",
            self.remark,
            now.format("%Y-%m-%d %H:%M:%S")
        );
    }

    /// 转换为数据库实体，用于数据库写入 (INSERT / UPDATE)
    pub fn into_entity(self) -> LiveStreamProviderEntity {
        LiveStreamProviderEntity {
            id: self.id,
            provider_code: self.provider_code,
            provider_name: self.provider_name,
            api_config: self.api_config,
            stream_config: self.stream_config,
            sort: self.sort,
            add_time: self.add_time,
            update_time: self.update_time,
            created_at: self.created_at,
            updated_at: self.updated_at,
            push_domain: self.push_domain,
            play_flv_domain: self.play_flv_domain,
            play_hls_domain: self.play_hls_domain,
            app_name: self.app_name,
            stream_secret: self.stream_secret,
            push_url_template: self.push_url_template,
            play_flv_template: self.play_flv_template,
            play_hls_template: self.play_hls_template,
            sdk_start_cmd: self.sdk_start_cmd,
            sdk_stop_cmd: self.sdk_stop_cmd,
            expire_seconds: self.expire_seconds,
            remark: self.remark,
            status: self.status,
        }
    }
}
