// video/src/router/info/api.rs  -- 用户状态模型
// 2026/4/23 13:42 by wx: cestbon10080

////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Normal,   // 正常状态
    Disabled, // 禁用状态
    Deleted,  // 删除状态
    Banned,
}

#[derive(Debug, Clone)]
pub struct UserState {
    pub status: UserStatus,
    pub banned: bool,
    pub uid: i64,
    pub is_playable: bool,
    pub status_code: Option<i16>,
}

impl UserState {
    /// 用户是否正常可用
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Normal
    }

    /// 是否被封禁
    pub fn is_banned(&self) -> bool {
        self.banned
    }

    /// 是否允许参与业务
    pub fn is_playable(&self) -> bool {
        self.is_active() && !self.is_banned()
    }
}
