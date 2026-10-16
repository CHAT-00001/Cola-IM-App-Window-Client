// cola_data/src/user/response/profile.rs  -- DATA - USER - RESPONSE - 资料
// 2026/5/21 03:47

////////

use crate::app::page::PageInfo;
use crate::cola_user::info::profile::ProfileInfo;
use crate::cola_user::info::user::UserInfo;
use serde::{Deserialize, Serialize};

////////

/// # [RESPONSE] - 用户主页社交统计
/// * `desc`: 用户主页公开展示的社交关系资产
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileSocialStats {
    pub fans_count: i64,      // 粉丝数量
    pub following_count: i64, // 关注数量
    pub friends_count: i64,   // 好友数量
}

////////

/// # [RESPONSE] - 用户主页内容统计
/// * `desc`: 用户发布内容获得的公开互动统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileContentStats {
    pub video_count: i64,         // 发布视频数量
    pub dynamic_count: i64,       // 发布动态数量
    pub video_likes_count: i64,   // 视频收到的点赞数量
    pub dynamic_likes_count: i64, // 动态收到的点赞数量
    pub total_likes_count: i64,   // 汇总获赞数量
    pub total_collect_count: i64, // 汇总被收藏数量
    pub total_view_count: i64,    // 汇总浏览数量
}

////////

/// # [RESPONSE] - 用户主页对象 Count
/// * `desc`: 主页 UGC 对象本身获得的行为统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileCount {
    pub view_count: i64,    // 主页访问数量
    pub like_count: i64,    // 主页点赞数量
    pub collect_count: i64, // 主页收藏数量
    pub share_count: i64,   // 主页分享数量
    pub comment_count: i64, // 主页评论数量
    pub follow_count: i64,  // 因主页产生的关注数量
}

////////

/// # [RESPONSE] - 用户主页对象关系
/// * `desc`: 当前访问者与目标用户主页 UGC 对象的关系
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileRelation {
    pub is_following: bool, // 是否关注目标用户
    pub is_blocked: bool,   // 是否拉黑目标用户
    pub is_visited: bool,   // 是否访问过主页
    pub is_liked: bool,     // 是否点赞主页
    pub is_collected: bool, // 是否收藏主页
    pub is_pushed: bool,    // 是否开启主页推送
}

////////

/// # [RESPONSE] - 用户主页视图对象
/// * `desc`: 以用户为主体的公开 UGC 聚合对象
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileVo {
    pub app_id: String,               // 预设应用标识
    pub biz_id: String,               // 预设业务标识
    #[serde(flatten)]
    pub user: UserInfo,               // 基础用户资料
    pub profile: Option<ProfileInfo>, // 公开资料名片
    pub social: ProfileSocialStats,   // 社交统计
    pub content: ProfileContentStats, // 内容统计
    pub count: ProfileCount,          // 主页对象 Count
    pub relation: ProfileRelation,    // 当前访问者关系
}

////////

/// # [BUILD] - 构建用户主页聚合对象
impl ProfileVo {
    pub fn combine(
        app_id: String,
        biz_id: String,
        user: UserInfo,
        profile: Option<ProfileInfo>,
        social: ProfileSocialStats,
        content: ProfileContentStats,
        count: ProfileCount,
        relation: ProfileRelation,
    ) -> Self {
        Self {
            app_id,
            biz_id,
            user,
            profile,
            social,
            content,
            count,
            relation,
        }
    }
}

impl Default for ProfileVo {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            biz_id: String::new(),
            user: UserInfo::default(),
            profile: None,
            social: ProfileSocialStats::default(),
            content: ProfileContentStats::default(),
            count: ProfileCount::default(),
            relation: ProfileRelation::default(),
        }
    }
}

/// # [RESPONSE] - 单对象响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileSingleResponse {
    pub info: ProfileVo, // 用户主页聚合对象
}

/// # [RESPONSE] - 多对象响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileListResponse {
    pub list: Vec<ProfileVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 空列表
impl ProfileListResponse {
    /// ✅ 创建一个空的列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }

    pub fn new(list: Vec<ProfileVo>, page: i64, qty: i64) -> Self {
        Self {
            list,
            page_info: PageInfo {
                page,
                qty,
                has_more: false,
            },
        }
    }
}

// 默认列表
impl Default for ProfileListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
