// cola_data/src/wallet/entity/users_manage.rs
// ✅ WALLET - 钱包用户/KYC管理实体
// 2026/8/20  Created.

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};

////////

/// # [ENTITY] - 钱包用户管理
/// * `pg schema`: `cola_wallet` - PG 模式
/// * `table_name`: `wallet_users_manage`
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WalletUserManage {
    pub id: i64,                // BIGSERIAL 主键
    pub user_id: i64,           // 关联主表 user.id
    pub wallet_user_no: String, // 钱包用户编号（如：WU + 时间戳 + 随机数）

    // 使用 Option 处理 NULL 字段
    pub real_name: Option<String>,        // 真实姓名
    pub id_card_no: Option<String>,       // 身份证号（加密存储）
    pub id_card_front: Option<String>,    // 身份证正面（OSS路径）
    pub id_card_back: Option<String>,     // 身份证反面（OSS路径）
    pub id_card_handheld: Option<String>, // 手持身份证照片（OSS路径）

    pub kyc_status: i16, // KYC状态：0-未认证 1-审核中 2-已认证 3-认证失败 4-已冻结
    pub kyc_level: i16,  // 认证等级：0-未认证 1-Lv1 2-Lv2 3-Lv3
    pub kyc_fail_reason: Option<String>, // 认证失败原因
    pub kyc_submitted_at: Option<DateTime<Utc>>, // 认证提交时间
    pub kyc_verified_at: Option<DateTime<Utc>>, // 认证通过时间

    pub pay_password: Option<String>, // 支付密码（bcrypt加密）
    pub pay_password_set: bool,       // 是否设置支付密码
    pub pay_password_fails: i16,      // 支付密码连续错误次数
    pub pay_password_locked: bool,    // 支付密码是否锁定
    pub pay_password_lock_at: Option<DateTime<Utc>>, // 锁定时间

    pub status: i16,                   // 状态：0-冻结 1-正常 2-注销
    pub freeze_reason: Option<String>, // 冻结原因

    pub daily_limit: Decimal,   // 单日限额（0=不限）
    pub single_limit: Decimal,  // 单笔限额（0=不限）
    pub monthly_limit: Decimal, // 单月限额（0=不限）
    pub today_used: Decimal,    // 今日已用额度
    pub month_used: Decimal,    // 本月已用额度

    pub last_transfer_at: Option<DateTime<Utc>>, // 最近一次转账时间
    pub last_login_ip: Option<String>,           // 最后登录IP
    pub last_login_at: Option<DateTime<Utc>>,    // 最后登录时间

    pub remark: Option<String>, // 备注
    pub extra: JsonValue,       // 扩展信息（JSONB）

    pub created_at: DateTime<Utc>, // 创建时间
    pub updated_at: DateTime<Utc>, // 更新时间
}
