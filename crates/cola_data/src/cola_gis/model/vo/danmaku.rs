// /vo/danmaku  -- VO - 弹幕
// 2026/5/22 15:23

////////

use cola_data::app::page::PageInfo;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use serde::Serialize;

////////

/// # [VO] - 弹幕 视图对象
#[derive(Serialize, Debug)] // 🌟 已修正：移除了不满足条件的 Default 派生
pub struct DanmakuVo {
    #[serde(flatten)]
    pub danmaku: DanmakuInfo, // 🌟 已修正：改回小写 snake_case 规范
    pub is_liked: bool,    // 是否点赞
    pub is_disliked: bool, // 是否不喜欢
    pub is_author: bool,   // 是否是视频作者发的
    pub is_own: bool,      // 是否是当前登录用户自己发的
}

/// # [BUILD] - 构造函数
impl DanmakuVo {
    /// 从已有的 DanmakuInfo 组装成最终的 VO 对象
    pub fn from_info(
        danmaku: DanmakuInfo,     // 🌟 已修正：去掉了 mut，不污染底层元数据
        current_uid: Option<i64>, // 传入当前登录用户的 UID (用来判断 is_own)
        video_author_id: i64,     // 传入视频作者的 UID (用来判断 is_author)
        is_liked: bool,           // 外部点赞状态
        is_disliked: bool,        // 外部不喜欢状态
    ) -> Self {
        // 1. 判断是不是当前登录用户自己发的弹幕
        let is_own = current_uid
            .map(|uid| uid == danmaku.user_id)
            .unwrap_or(false);

        // 2. 判断是不是视频作者发的弹幕
        let is_author = danmaku.user_id == video_author_id;

        // 🌟 已修正：删除了对 danmaku.is_author 的非法赋值，保持 Info 干净

        Self {
            danmaku, // 🌟 已修正：对应小写字段
            is_liked,
            is_disliked,
            is_author,
            is_own,
        }
    }
}

/// # [RESPONSE] - 单条弹幕响应 (适用于：发布弹幕成功后返回、查看单条弹幕详情)
#[derive(Serialize, Debug)]
pub struct DanmakuSingleResponse {
    pub info: DanmakuVo,
}

/// # [RESPONSE] - 弹幕列表响应 (适用于：视频底下的弹幕区分页列表)
#[derive(Serialize, Debug)]
pub struct DanmakuListResponse {
    pub danmakus: Vec<DanmakuVo>, // 弹幕列表
    pub page_info: PageInfo,      // 分页信息
}

//////// END
