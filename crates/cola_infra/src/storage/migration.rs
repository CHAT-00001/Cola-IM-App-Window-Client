// src/storage/migration.rs -- 存储层 - 版本化迁移 (PRAGMA user_version)
// 2026/10/16 08:00 Created.

////////

use rusqlite::Connection;

use crate::error::{Result, SdkError};

////////

/// # [MIGRATION] - 迁移步骤列表
/// * `desc`: `按序号递增追加, 已发布的步骤不得修改, 只能新增`
/// * `condition`: `数组下标 + 1 即为目标 user_version`
const MIGRATIONS: &[&str] = &[
    // v1: 验证码记录表 (登录前数据, 按 application_id 隔离)
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
];

/// # 1. [MIGRATION] - 执行未完成的迁移
/// * `desc`: `读取 user_version, 依次执行未应用的步骤并更新版本号`
/// * `condition`: `数据库版本高于当前代码支持的版本时返回错误, 避免降级破坏数据`
pub fn migrate(conn: &mut Connection) -> Result<()> {
    let current: usize = conn
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(storage_err)? as usize;
    let latest = MIGRATIONS.len();

    if current > latest {
        return Err(SdkError::Storage(format!(
            "database version {current} is newer than supported {latest}"
        )));
    }

    for (index, sql) in MIGRATIONS.iter().enumerate().skip(current) {
        let version = index + 1;
        let tx = conn.transaction().map_err(storage_err)?;
        tx.execute_batch(sql).map_err(storage_err)?;
        tx.pragma_update(None, "user_version", version as i64)
            .map_err(storage_err)?;
        tx.commit().map_err(storage_err)?;
        tracing::info!("[🗣️ STORAGE MIGRATION]: ✅️ 已应用迁移 v{}", version);
    }
    Ok(())
}

////////

/// # [MIGRATION] - 统一映射 SQLite 错误
/// * `desc`: `把 rusqlite 错误转换为 SDK 统一存储错误`
fn storage_err(error: rusqlite::Error) -> SdkError {
    SdkError::Storage(error.to_string())
}

////////

#[cfg(test)]
mod tests {
    use super::*;

    fn user_version(conn: &Connection) -> i64 {
        conn.query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn migrates_fresh_database_to_latest_version() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(user_version(&conn), MIGRATIONS.len() as i64);
    }

    #[test]
    fn migrate_is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        migrate(&mut conn).unwrap();
        assert_eq!(user_version(&conn), MIGRATIONS.len() as i64);
    }

    #[test]
    fn rejects_database_newer_than_supported() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", 99i64).unwrap();
        assert!(migrate(&mut conn).is_err());
    }
}

//////// END
