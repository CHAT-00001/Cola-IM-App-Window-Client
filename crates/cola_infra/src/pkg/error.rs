// video/src/live/pkg/error.rs  --
// 2026/3/28 08:13 by wx: cestbon10080

////////

use data::app::data::AppData;

/// # PKG
#[derive(Debug)]
pub enum AppError {
    ParamsError(String),    // 4000: 参数错误
    VideoNotFound,          // 4004: 视频不存在
    PermissionDenied,       // 4003: 权限不足
    Internal(String),       // 5000: 系统内部错误
}

impl AppError {
    // 将业务错误映射为你的 AppData 结构
    pub fn to_response(self) -> AppData<()> {
        match self {
            AppError::ParamsError(msg) => AppData::err(4000, msg, None),
            AppError::VideoNotFound => AppData::err(4004, "视频不存在或已下架", None),
            AppError::PermissionDenied => AppData::err(4003, "无权操作", None),
            AppError::Internal(e) => AppData::err(5000, e, None),
        }
    }
}