// /port/like.rs  --
// 2026/6/10 06:15

////////

/// # [PORT] - 点赞
#[async_trait::async_trait]
pub trait VideoLikeRepo: Send + Sync {
    ////////

    /// # [PORT] - 点赞
    async fn like_video(&self, uid: i64, video_id: i64, is_liked: bool) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 不喜欢
    async fn unlike_video(&self, uid: i64, video_id: i64, is_unliked: bool) -> anyhow::Result<()>;
}
