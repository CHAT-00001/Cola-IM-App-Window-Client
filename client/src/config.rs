//! Runtime configuration loaded from the project's `.env` file.
//!
//! `.env` is deliberately *not* embedded in the binary: it is read from disk on
//! startup, so it can be edited (and the app restarted) without recompiling.
//!
//! Lookup order for the file itself:
//!
//! 1. `<current working directory>/.env` — e.g. the repository root for
//!    `cargo run -p client`
//! 2. `<directory of the running executable>/.env` — for a copied/shipped binary
//! 3. `<crate root>/.env` — `client/.env`, baked in at compile time
//! 4. `<workspace root>/.env` — the repository root, also baked in, so
//!    `cargo run --release -p client`, a double-clicked
//!    `target/release/cola-im-desktop-client.exe`, `cargo test` from `client/`
//!    and any other working directory all still pick up the repository's `.env`
//!
//! A real environment variable with the same name always wins over the file.
//! The resolved FPS switch is printed to stderr at startup, e.g.
//! `[cola] FPS 帧率表：开（…\.env 中的 FPS_DISPLAY_MODE=1）`.
//!
//! Everything here is independent of debug/release: `FPS_DISPLAY_MODE=1` shows
//! the meter, `0` hides it, in both profiles.

use std::path::PathBuf;

/// `FPS_DISPLAY_MODE=1` switches the on-screen FPS meter on; `0` (or anything
/// else) keeps it off.
const FPS_MODE_KEY: &str = "FPS_DISPLAY_MODE";

/// Variable understood before `.env` existed; still honoured as a fallback.
const FPS_MODE_KEY_LEGACY: &str = "COLA_FPS_OVERLAY";

/// Whether the FPS meter should be drawn, plus the reason (logged at startup so
/// "为什么没显示" can be answered from stderr alone).
pub fn fps_display_enabled() -> bool {
    let (enabled, source) = resolve_fps_switch();
    log_startup(&format!(
        "FPS 帧率表：{}（{source}）",
        if enabled { "开" } else { "关" }
    ));
    enabled
}

fn resolve_fps_switch() -> (bool, String) {
    // 1. A real environment variable wins over the file.
    if let Ok(value) = std::env::var(FPS_MODE_KEY) {
        return (is_on(&value), format!("环境变量 {FPS_MODE_KEY}={value}"));
    }
    // 2. `.env` (`FPS_DISPLAY_MODE = 1`).
    if let Some((path, entries)) = read_env_file()
        && let Some((_, value)) = entries.iter().find(|(key, _)| key == FPS_MODE_KEY)
    {
        return (
            is_on(value),
            format!("{} 中的 {FPS_MODE_KEY}={value}", path.display()),
        );
    }
    // 3. Variable understood before `.env` existed.
    if let Ok(value) = std::env::var(FPS_MODE_KEY_LEGACY) {
        return (
            is_on(&value),
            format!("环境变量 {FPS_MODE_KEY_LEGACY}={value}"),
        );
    }
    (false, format!("没有找到 {FPS_MODE_KEY}"))
}

/// `1`, `true`, `on` and `yes` (any case) mean "enabled".
fn is_on(value: &str) -> bool {
    let value = value.trim();
    value == "1"
        || value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("yes")
}

/// The first `.env` file that exists, with its path and parsed `KEY = VALUE` pairs.
fn read_env_file() -> Option<(PathBuf, Vec<(String, String)>)> {
    for path in env_file_candidates() {
        if let Ok(contents) = std::fs::read_to_string(&path) {
            return Some((path, parse_env(&contents)));
        }
    }
    None
}

/// Lookup order: current working directory → directory of the running
/// executable → the crate root (`client/`) → the workspace root (repository
/// root) recorded at compile time.
///
/// The workspace-root entry exists because the package lives in `client/` while
/// the runtime `.env` stays at the repository root: `cargo run -p client` from
/// anywhere, a double-clicked `target/release/cola-im-desktop-client.exe` and
/// `cargo test` all still find it.
fn env_file_candidates() -> Vec<PathBuf> {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut paths = vec![PathBuf::from(".env")];
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        paths.push(dir.join(".env"));
    }
    paths.push(crate_root.join(".env"));
    if let Some(workspace_root) = crate_root.parent() {
        paths.push(workspace_root.join(".env"));
    }
    paths
}

/// Minimal `.env` parser: one `KEY = value` per line, `#` starts a comment and
/// surrounding quotes are stripped. Enough for the handful of switches this
/// demo reads, without pulling in a dotenv crate.
fn parse_env(contents: &str) -> Vec<(String, String)> {
    contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            if key.is_empty() {
                return None;
            }
            Some((key.to_string(), unquote(value.trim())))
        })
        .collect()
}

fn unquote(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && (bytes[0] == b'"' || bytes[0] == b'\'')
        && bytes[bytes.len() - 1] == bytes[0]
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

fn log_startup(message: &str) {
    if std::env::var_os("COLA_QUIET").is_none() {
        eprintln!("[cola] {message}");
    }
}

#[cfg(test)]
mod tests {
    use super::{FPS_MODE_KEY, env_file_candidates, fps_display_enabled, is_on, parse_env, read_env_file};
    use std::path::PathBuf;

    #[test]
    fn parses_spaces_comments_and_quotes() {
        let parsed = parse_env(
            "# comment\nFPS_DISPLAY_MODE = 1\nQUOTED=\"a b\"\nEMPTY=\nNOSEP\n#FPS_DISPLAY_MODE=0\n",
        );
        assert_eq!(
            parsed,
            vec![
                ("FPS_DISPLAY_MODE".to_string(), "1".to_string()),
                ("QUOTED".to_string(), "a b".to_string()),
                ("EMPTY".to_string(), String::new()),
            ]
        );
    }

    #[test]
    fn enabled_values() {
        assert!(is_on("1"));
        assert!(is_on(" TRUE "));
        assert!(!is_on("0"));
        assert!(!is_on("off"));
    }

    /// `.env` 在仓库根目录（工作区根），无论 `cargo test` 的工作目录是仓库根还是
    /// `client/`，都必须被找到，并且 `fps_display_enabled()` 的结果要与文件里的开关一致。
    #[test]
    fn dot_env_at_the_workspace_root_drives_the_fps_switch() {
        let (path, entries) = read_env_file().expect(".env should be found from the workspace root");
        assert!(path.ends_with(".env"), "unexpected path {path:?}");
        // 候选项里必须包含工作区根（仓库根），否则不在仓库根运行时就找不到配置。
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the crate lives in a workspace directory")
            .join(".env");
        assert!(
            env_file_candidates().contains(&workspace_root),
            "workspace root candidate missing: {workspace_root:?}"
        );
        let from_file = entries
            .iter()
            .find(|(key, _)| key == FPS_MODE_KEY)
            .map(|(_, value)| is_on(value))
            .unwrap_or(false);
        assert_eq!(fps_display_enabled(), from_file);
    }
}
