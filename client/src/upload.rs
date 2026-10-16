// client/upload.rs -- S3 上载器
// 2026-10-15 22:40

////////

//! 预留的 S3 异步上传能力。
//!
//! 现在附件（照片 / 视频 / 文件）只保存在内存里，气泡直接用本地路径渲染缩略图；
//! 这一层是给后期接入 S3 留的插槽：
//!
//! 1. 发送附件时 `Messenger` 会把 [`UploadRequest`] 交给 [`S3Uploader`]，
//!    在 GPUI 的前台执行器上异步跑（不阻塞渲染），完成后回写 [`UploadState`]；
//! 2. 后期接入时只需要把 [`StubUploader`] 换成真实实现
//!    （例如 `reqwest` 预签名 URL PUT，或 aws-sdk-s3），
//!    `Messenger::new` 里换一个 `Arc<dyn S3Uploader>` 即可，界面代码不用动。
//!
//! 换实现时要保持的约定：
//! - `upload()` 必须是纯异步的，绝不能阻塞调用线程（渲染在主线程上）；
//! - `Ok(String)` 返回可直接写进消息体的对象 URL（CDN / bucket 地址）。

use std::path::{Path, PathBuf};

use futures::future::BoxFuture;

/// 附件在 S3 上的上传状态。
///
/// 目前界面不展示它（只保存在内存里），字段与状态机先留好，
/// 后期接上真实 S3 后直接在气泡角标上读 `is_uploaded()` / `url()` 即可。
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum UploadState {
    /// 刚创建、还没开始上传。
    #[default]
    Pending,
    /// 已交给上传器，等待结果。
    Uploading,
    /// 上传成功，值是对象 URL（写进消息经 SDK 发给对方）。
    Uploaded { url: String },
    /// 上传失败（S3 未配置 / 网络错误），值是原因。
    Failed { reason: String },
}

#[allow(dead_code)]
impl UploadState {
    /// 是否已经拿到对象 URL。
    pub fn is_uploaded(&self) -> bool {
        matches!(self, Self::Uploaded { .. })
    }

    /// 成功后的对象 URL。
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Uploaded { url } => Some(url),
            _ => None,
        }
    }
}

/// 一次上传请求：本地文件 + 目标对象键。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UploadRequest {
    /// 待上传的本地文件。
    pub path: PathBuf,
    /// 桶里的对象键，例如 `im/1760603000000000000-photo.png`。
    pub key: String,
    /// 上传时带的 Content-Type，例如 `image/png`。
    pub content_type: String,
}

impl UploadRequest {
    /// 按本地路径生成对象键与 Content-Type。
    ///
    /// 对象键用「纳秒时间戳 + 文件名」，保证同一张图重复发送不会互相覆盖；
    /// 文件名里的路径分隔符会被去掉，避免上传出多余的目录层级。
    pub fn for_path(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            key: object_key(path),
            content_type: content_type_for(path),
        }
    }
}

/// # [PORT] - 预留的 S3 上传接口。
///
/// `Send + Sync` 是为了能放进 `Arc` 里跨线程/跨 await 点持有；
/// 返回 `BoxFuture` 是为了保持 trait object 安全（`async fn` 在 trait 里不行）。
pub trait S3Uploader: Send + Sync + 'static {
    /// 把 `request.path` 上传到 `request.key`，成功时返回可访问的对象 URL。
    fn upload(&self, request: UploadRequest) -> BoxFuture<'static, Result<String, String>>;
}

/// 占位实现：S3 还没接入，异步返回失败，界面保持在内存里的本地渲染。
///
/// 换成真实实现时不要改这里，直接新增一个 `struct S3Client` 并在
/// `Messenger::new` 里替换即可（本结构体可留作离线 / 单测用）。
#[derive(Clone, Copy, Debug, Default)]
pub struct StubUploader;

impl StubUploader {
    pub fn new() -> Self {
        Self
    }
}

////////

/// # [UPLOADER]  - 文件上载器
impl S3Uploader for StubUploader {
    //

    ////////

    // # [ACTION] - 上载
    fn upload(&self, request: UploadRequest) -> BoxFuture<'static, Result<String, String>> {
        Box::pin(async move {
            Err(format!(
                "尚未接入 S3：{} 只保存在内存中",
                request.path.display()
            ))
        })
    }
}

////////

/// 对象键：`im/<纳秒时间戳>-<文件名>`。
pub fn object_key(path: &Path) -> String {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    format!("im/{stamp}-{}", file_name(path))
}

////////

/// 按扩展名给出 Content-Type；未知扩展名回退到 `application/octet-stream`。
pub fn content_type_for(path: &Path) -> String {
    let extension = path
        .extension()
        .map(|extension| extension.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    let mime = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        "mkv" => "video/x-matroska",
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        "zip" => "application/zip",
        _ => "application/octet-stream",
    };
    mime.to_string()
}

////////

/// # [BUILD] - 文件名称
fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().replace(['/', '\\'], "_"))
        .unwrap_or_else(|| "attachment".to_string())
}


//////// TEST

//
#[cfg(test)]
mod tests {
    use super::{
        S3Uploader, StubUploader, UploadRequest, UploadState, content_type_for, object_key,
    };
    use std::path::PathBuf;

    #[test]
    fn upload_state_keeps_the_object_url() {
        let pending = UploadState::default();
        assert_eq!(pending, UploadState::Pending);
        assert!(!pending.is_uploaded());
        assert_eq!(pending.url(), None);

        let uploaded = UploadState::Uploaded {
            url: "https://cdn.example.com/im/a.png".to_string(),
        };
        assert!(uploaded.is_uploaded());
        assert_eq!(uploaded.url(), Some("https://cdn.example.com/im/a.png"));

        assert!(matches!(
            UploadState::Failed {
                reason: "网络错误".to_string()
            },
            UploadState::Failed { .. }
        ));
    }

    #[test]
    fn content_types_follow_the_extension() {
        assert_eq!(
            content_type_for(&PathBuf::from("a/b/PHOTO.PNG")),
            "image/png"
        );
        assert_eq!(content_type_for(&PathBuf::from("clip.mp4")), "video/mp4");
        assert_eq!(
            content_type_for(&PathBuf::from("no-extension")),
            "application/octet-stream"
        );
    }

    #[test]
    fn object_keys_keep_the_file_name_without_directories() {
        let key = object_key(&PathBuf::from(r"C:\photos\trip\a.png"));
        assert!(key.starts_with("im/"), "unexpected key {key}");
        assert!(key.ends_with("-a.png"), "unexpected key {key}");
        assert!(!key.contains('\\'), "unexpected key {key}");
    }

    #[test]
    fn the_stub_uploader_reports_that_s3_is_not_wired_yet() {
        let request = UploadRequest::for_path(&PathBuf::from("a.png"));
        assert_eq!(request.content_type, "image/png");
        let result = gpui_kit::block_on(StubUploader::new().upload(request));
        assert!(result.is_err());
    }
}

//////// END