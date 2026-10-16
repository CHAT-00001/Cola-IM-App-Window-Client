// src/storage/entity/captcha.rs -- 存储层 - 实体 - 验证码记录 (captcha_record)
// 2026/10/16 08:00 Created.

////////

use rusqlite::Row;

////////

/// # [ENTITY] - 验证码记录
/// * `table name`: `captcha_record`
/// * `desc`: `登录前数据, 按 application_id 隔离, 不带 account_id`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CaptchaEntity {
    pub id: i64,                // 主键 ID (自增)
    pub application_id: String, // 应用 ID
    pub area: String,           // 区号 "+86"
    pub phone_no: String,       // 手机号
    pub http_status: i64,       // HTTP 状态码
    pub response_json: String,  // 服务端原始 JSON 文本
    pub created_at: i64,       // 创建时间 (unix 秒)
}

/// # [ENTITY] - 验证码表统一列名 (与 CaptchaEntity 字段顺序一致)
pub const CAPTCHA_COLUMNS: &str =
    "id, application_id, area, phone_no, http_status, response_json, created_at";

impl CaptchaEntity {
    /// # 1. [ENTITY] - 从行映射实体
    /// * `desc`: `按 CAPTCHA_COLUMNS 的列顺序读取一行数据`
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            application_id: row.get(1)?,
            area: row.get(2)?,
            phone_no: row.get(3)?,
            http_status: row.get(4)?,
            response_json: row.get(5)?,
            created_at: row.get(6)?,
        })
    }
}

//////// END
