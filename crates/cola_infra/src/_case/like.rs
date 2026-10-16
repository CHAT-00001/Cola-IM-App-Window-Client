// video/src/live/case/like.rs  -- 点赞
// 2026-03-27 06:22

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use repo::video::service::like::LikeService;
use tracing::{info, warn};

////////

/// # [USE CASE] - 点赞 用例
pub struct LikeCase;

impl LikeCase {
    ////////

    ////////

    /// # 1. [CASE] - 喜欢
    pub async fn case_add_video_like(uid: i64, video_id: i64, is_liked: bool) -> Result<()> {
        // 1. 调用 Service 层
        // 这里使用了 map_err 将数据库错误转换为 anyhow 的 Result，保证外层处理逻辑一致
        LikeService::save_like_with_update_video_count(uid, video_id, is_liked)
            .await
            .map_err(|e| {
                anyhow!(
                    "系统错误: 点赞处理失败 (uid: {}, vid: {}, err: {})",
                    uid,
                    video_id,
                    e
                )
            })?;

        // 2. 日志记录
        // 根据 is_like 的状态输出更具体的日志，方便后续排查问题
        let action = if is_liked { "点赞" } else { "取消点赞" };
        info!("用户 {} {}了视频 {}", uid, action, video_id);

        Ok(())
    }

    ////////

    /// # 3. [CASE] - 不喜欢
    pub async fn case_add_video_unlike(uid: i64, video_id: i64, is_like: bool) -> Result<()> {
        // 1. 调用 Service 层
        // 这里使用了 map_err 将数据库错误转换为 anyhow 的 Result，保证外层处理逻辑一致
        LikeService::save_like_with_update_video_count(uid, video_id, is_like)
            .await
            .map_err(|e| {
                anyhow!(
                    "系统错误: 不喜欢处理失败 (uid: {}, vid: {}, err: {})",
                    uid,
                    video_id,
                    e
                )
            })?;

        // 2. 日志记录
        // 根据 is_like 的状态输出更具体的日志，方便后续排查问题
        let action = if is_like {
            "不喜欢"
        } else {
            "取消不喜欢"
        };
        info!("用户 {} {}了视频 {}", uid, action, video_id);

        Ok(())
    }
}

//////// END
