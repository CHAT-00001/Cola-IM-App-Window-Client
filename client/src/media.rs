//! 消息附件：照片 / 视频 / 文件的数据，以及气泡里缩略图的尺寸规则。
//!
//! 缩略图规则（与微信一致）：
//!
//! - 边界固定 **120x120**（[`THUMB_MAX`]）；
//! - **不裁切**（渲染时用 `ObjectFit::Contain`），等比缩放；
//! - 只缩小不放大，最长边贴到 400 就停，另一边按比例跟着缩；
//! - 图片尺寸读不出来时按 400x400 的方块兜底，保证气泡不会忽大忽小。

use std::path::{Path, PathBuf};

use crate::upload::UploadState;

/// 缩略图边界：宽、高都不超过它。
pub const THUMB_MAX: f32 = 120.0;

/// 附件类型。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKind {
    /// 照片（png / jpg / gif / webp …）。
    Image,
    /// 视频（mp4 / mov / avi / mkv …）。
    Video,
    /// 其它文件。
    File,
}

impl MediaKind {
    /// 界面上的中文名（按钮提示、文件卡片标题都用它）。
    pub fn label(self) -> &'static str {
        match self {
            Self::Image => "照片",
            Self::Video => "视频",
            Self::File => "文件",
        }
    }

    /// 文件卡片左侧的小图标。
    pub fn icon(self) -> &'static str {
        match self {
            Self::Image => "▧",
            Self::Video => "▣",
            Self::File => "▤",
        }
    }
}

/// 按扩展名判断附件类型（GPUI 的文件选择器没有类型过滤，选完再按后缀归类）。
pub fn kind_of_path(path: &Path) -> Option<MediaKind> {
    let extension = path.extension()?.to_string_lossy().to_ascii_lowercase();
    match extension.as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "ico" | "tif" | "tiff" | "avif" | "qoi"
        | "svg" => Some(MediaKind::Image),
        "mp4" | "mov" | "m4v" | "avi" | "mkv" | "webm" | "wmv" | "flv" | "mpg" | "mpeg" => {
            Some(MediaKind::Video)
        }
        "pdf" | "txt" | "md" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "zip" | "rar"
        | "7z" => Some(MediaKind::File),
        _ => None,
    }
}

/// 一条消息里的附件；本地路径 + 元数据，全部留在内存里。
#[derive(Clone, Debug)]
pub struct Attachment {
    pub kind: MediaKind,
    /// 本地文件路径；`img()` 直接从磁盘读它渲染缩略图。
    pub path: PathBuf,
    /// 文件名（气泡里显示）。
    pub name: String,
    /// 字节数。
    pub size: u64,
    /// 图片的像素宽；读不出来或非图片时为 0。
    pub width: u32,
    /// 图片的像素高；读不出来或非图片时为 0。
    pub height: u32,
    /// 预留：S3 上传状态（见 `client/src/upload.rs`）。
    pub upload: UploadState,
}

impl Attachment {
    /// 从本地路径建附件：读文件名 / 大小，图片再探一次像素尺寸。
    ///
    /// 只读文件头（`image::image_dimensions`），不会整张解码，所以放在渲染线程上也不会卡。
    pub fn from_path(kind: MediaKind, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let (width, height) = match kind {
            // 视频暂时没有抽帧能力，读不到尺寸就退回 400x400 的方块。
            MediaKind::Image | MediaKind::Video => image_dimensions(&path).unwrap_or((0, 0)),
            MediaKind::File => (0, 0),
        };
        Self {
            kind,
            name: file_name(&path),
            size: file_size(&path),
            width,
            height,
            upload: UploadState::Pending,
            path,
        }
    }

    /// 气泡里缩略图 / 文件卡片的显示尺寸（逻辑像素）。
    pub fn display_size(&self) -> (f32, f32) {
        thumbnail_size(self.width, self.height)
    }
}

/// 边界 400x400 的等比缩放：`min(1, 400/w, 400/h)`。
///
/// 先算「宽贴边」和「高贴边」两个比例，取小的那个，所以先碰到边界的那条边就是 400；
/// 再和 1 取 min，保证小图不会被放大。`width` / `height` 为 0（读不出尺寸）时用 400x400 兜底。
///
/// 最后再 `min(THUMB_MAX)` 夹一次：f32 乘法会有极小误差（例如 1000*0.4 = 400.00003），
/// 夹一下才能保证「边界就是上限」，不会因为 0.00003 像素把布局顶出框。
pub fn thumbnail_size(width: u32, height: u32) -> (f32, f32) {
    if width == 0 || height == 0 {
        return (THUMB_MAX, THUMB_MAX);
    }
    let scale = (THUMB_MAX / width as f32)
        .min(THUMB_MAX / height as f32)
        .min(1.0);
    (
        (width as f32 * scale).min(THUMB_MAX),
        (height as f32 * scale).min(THUMB_MAX),
    )
}

/// 图片像素尺寸；非图片 / 损坏 / 无权限都返回 `None`。
pub fn image_dimensions(path: &Path) -> Option<(u32, u32)> {
    image::image_dimensions(path).ok()
}

/// 文件名（含扩展名），取不到时给个占位。
pub fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "未命名".to_string())
}

/// 文件字节数；读不到（文件已删除等）时为 0。
pub fn file_size(path: &Path) -> u64 {
    std::fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

/// 人类可读的大小，例如 `1.2 MB`。
pub fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.;
    const MB: f64 = KB * 1024.;
    const GB: f64 = MB * 1024.;
    let bytes = bytes as f64;
    if bytes >= GB {
        format!("{:.1} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes / KB)
    } else {
        format!("{} B", bytes as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::{MediaKind, THUMB_MAX, human_size, kind_of_path, thumbnail_size};
    use std::path::PathBuf;

    /// 长边贴 400，短边等比；已经比 400 小的图不放大。
    #[test]
    fn thumbnails_keep_the_aspect_ratio_inside_the_400_boundary() {
        // 横图：宽先贴边。
        assert_eq!(thumbnail_size(800, 600), (400., 300.));
        // 竖图：高先贴边。
        assert_eq!(thumbnail_size(600, 800), (300., 400.));
        // 正好 400x400。
        assert_eq!(thumbnail_size(400, 400), (400., 400.));
        // 正方形大图。
        assert_eq!(thumbnail_size(1000, 1000), (400., 400.));
        // 只缩一条边。
        assert_eq!(thumbnail_size(400, 1000), (160., 400.));
        // 小图不放大。
        assert_eq!(thumbnail_size(200, 100), (200., 100.));
        // 1 像素也不越界。
        assert_eq!(thumbnail_size(1, 1), (1., 1.));
    }

    #[test]
    fn thumbnails_never_exceed_the_boundary_and_never_crop() {
        for (width, height) in [(1920, 1080), (1080, 1920), (3000, 7), (7, 3000)] {
            let (w, h) = thumbnail_size(width, height);
            assert!(w <= THUMB_MAX && h <= THUMB_MAX, "{width}x{height} -> {w}x{h}");
            // 等比：目标宽高比与原始一致（不裁切）。
            let source_ratio = width as f32 / height as f32;
            let target_ratio = w / h;
            assert!(
                (source_ratio - target_ratio).abs() < 0.001,
                "{width}x{height} -> {w}x{h}"
            );
        }
    }

    #[test]
    fn unknown_dimensions_fall_back_to_a_square_box() {
        assert_eq!(thumbnail_size(0, 0), (THUMB_MAX, THUMB_MAX));
        assert_eq!(thumbnail_size(0, 600), (THUMB_MAX, THUMB_MAX));
    }

    #[test]
    fn kinds_come_from_the_extension() {
        assert_eq!(kind_of_path(&PathBuf::from("a/b/C.JPG")), Some(MediaKind::Image));
        assert_eq!(kind_of_path(&PathBuf::from("clip.mp4")), Some(MediaKind::Video));
        assert_eq!(kind_of_path(&PathBuf::from("notes.txt")), Some(MediaKind::File));
        assert_eq!(kind_of_path(&PathBuf::from("weird")), None);
        assert_eq!(MediaKind::Image.label(), "照片");
        assert_eq!(MediaKind::Video.icon(), "▣");
    }

    #[test]
    fn sizes_are_human_readable() {
        assert_eq!(human_size(512), "512 B");
        assert_eq!(human_size(2048), "2.0 KB");
        assert_eq!(human_size(3 * 1024 * 1024), "3.0 MB");
    }
}

