// cola_data/src/cola_gis/cmd/poi.rs  -- GIS - Command - 发布兴趣点
// 2026/7/6 22:00

////////

use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 兴趣点 创建命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoiCommand {
    pub user_id: i64,                // 发布者ID
    pub title: String,               // 标题
    pub href: String,                //
    pub description: Option<String>, // 描述
    pub lat: Option<f64>,            // 纬度
    pub lng: Option<f64>,            // 经度
    pub tags: Vec<String>,           // tag
    pub category_id: i16,            // 分类ID
    pub cover_url: String,           //  封面URL
    pub thumb: String,               // 封面
    pub visibility_perm: i16,        // 可见权限
    pub comment_perm: i16,           // 评论权限
    pub danmaku_perm: i16,           // 弹幕权限
    pub collect_perm: i16,           // 收藏权限
    pub download_perm: i16,          // 下载权限
}

// 构造

impl PoiCommand {
    ////////

    /// # 1. [BUILD] - 新的
    pub fn new(self) {}

    /// # [ENTITY]
    pub fn into_entity(
        self,
        real_uid: i64,
        real_poi_id: i64,
    ) -> crate::cola_gis::entity::poi::PoiEntity {
        let now_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let get_perm = |client_perm: i16| -> i16 { if client_perm > 0 { client_perm } else { 5 } };

        crate::cola_gis::entity::poi::PoiEntity {
            id: real_poi_id,
            uid: real_uid,
            title: self.title,
            description: self.description,
            thumbnail: Some(self.cover_url),
            href: self.href,
            add_time: now_ts,
            lat: self.lat,
            lng: self.lng,
            visibility_perm: get_perm(self.visibility_perm),
            comment_perm: get_perm(self.comment_perm),
            danmaku_perm: get_perm(self.danmaku_perm),
            collect_perm: get_perm(self.collect_perm),
            download_perm: get_perm(self.download_perm),
            ..Default::default()
        }
    }
}

//////// END
