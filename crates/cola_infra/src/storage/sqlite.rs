// src/storage/sqlite.rs -- 存储层 - SQLite 仓储 (验证码记录)
// 2026/10/16 07:10 Created.

////////

use std::path::Path;

use rusqlite::{Connection, params};

use crate::error::{Result, SdkError};

////////

/// # [REPOSITORY] - SQLite 仓储
/// * `desc`: `管理本地 SQLite 连接并初始化表结构`
pub struct SqliteStore {
    conn: Connection, // SQLite 连接
}

impl SqliteStore {
    /// # 1. [REPOSITORY] - 打开内存数据库
    /// * `desc`: `创建仅存在于内存中的 SQLite 数据库, 用于测试`
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().map_err(storage_err)?;
        Self::init(conn)
    }

    /// # 2. [REPOSITORY] - 打开文件数据库
    /// * `desc`: `打开或创建本地 SQLite 文件并初始化表结构`
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path).map_err(storage_err)?;
        Self::init(conn)
    }

    /// # 3. [REPOSITORY] - 初始化表结构
    /// * `desc`: `创建验证码记录表, 按 application_id 隔离`
    fn init(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS captcha_record (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                application_id TEXT NOT NULL,
                area TEXT NOT NULL,
                phone_no TEXT NOT NULL,
                http_status INTEGER NOT NULL,
                response_json TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))
            );
            CREATE INDEX IF NOT EXISTS idx_captcha_app
                ON captcha_record (application_id, area, phone_no);",
        )
        .map_err(storage_err)?;
        Ok(Self { conn })
    }

    /// # 4. [REPOSITORY] - 写入验证码响应记录
    /// * `desc`: `把服务端返回的原始 JSON 按应用与手机号落库, 返回记录 ID`
    pub fn insert_captcha(
        &self,
        application_id: &str, // 应用 ID (验证码为登录前数据, 不带账号)
        area: &str,           // 区号
        phone_no: &str,       // 手机号
        http_status: i64,     // HTTP 状态码
        response_json: &str,  // 服务端返回的原始 JSON 文本
    ) -> Result<i64> {
        self.conn
            .execute(
                "INSERT INTO captcha_record (application_id, area, phone_no, http_status, response_json)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![application_id, area, phone_no, http_status, response_json],
            )
            .map_err(storage_err)?;
        Ok(self.conn.last_insert_rowid())
    }

    /// # 5. [REPOSITORY] - 查询最近一次验证码响应
    /// * `desc`: `按应用与手机号读取最新一条验证码响应 JSON`
    /// * `condition`: `无记录时返回 None`
    pub fn latest_captcha_response(
        &self,
        application_id: &str, // 应用 ID
        area: &str,           // 区号
        phone_no: &str,       // 手机号
    ) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT response_json FROM captcha_record
                 WHERE application_id = ?1 AND area = ?2 AND phone_no = ?3
                 ORDER BY id DESC LIMIT 1",
            )
            .map_err(storage_err)?;
        let mut rows = stmt
            .query(params![application_id, area, phone_no])
            .map_err(storage_err)?;
        match rows.next().map_err(storage_err)? {
            Some(row) => Ok(Some(row.get(0).map_err(storage_err)?)),
            None => Ok(None),
        }
    }
}

////////

/// # [REPOSITORY] - 统一映射 SQLite 错误
/// * `desc`: `把 rusqlite 错误转换为 SDK 统一存储错误`
fn storage_err(error: rusqlite::Error) -> SdkError {
    SdkError::Storage(error.to_string())
}

//////// END
