// data/src/cola_gis/info/poi.rs
// 🗄 数据 - 可乐GIS - info - POI信息
// 2026/7/6 20:40 Created.

////////

use crate::cola_gis::entity::poi::PoiEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - POI 信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoiInfo {
    pub id: i64,
    pub uid: i64,
    pub channel_id: i16,
    pub title: String,
    pub thumb: Option<String>,
    pub href: String,
    pub views: i32,
    pub likes: i32,
    pub collects: i32,
    pub shares: i32,
    pub visibility_perm: i16,
    pub comment_perm: i16,
    pub danmaku_perm: i16,
    pub download_perm: i16,
    pub add_time: i64,
}

// 构造实现
impl PoiInfo {
    //

    ////////

    /// # [BUILD] - 空的
    /// * `desc`: `兜底数据`
    pub fn empty() -> Self {
        Self {
            id: 0,
            title: "兴趣点不存在".to_string(),
            uid: 0,
            channel_id: 0,
            thumb: Option::from(String::new()),
            href: String::new(),
            views: 0,
            likes: 0,
            collects: 0,
            shares: 0,
            visibility_perm: 0,
            comment_perm: 0,
            danmaku_perm: 0,
            download_perm: 0,
            add_time: 0,
        }
    }

    ////////

    /// # [FROM] - 实体转换
    /// * `desc`: `转换对象`
    pub fn from_entity(entity: PoiEntity) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            channel_id: entity.channel_id.unwrap_or_default(),
            title: entity.title,
            thumb: Option::from(entity.thumb),
            href: entity.href,
            views: entity.views,
            likes: entity.likes,
            collects: entity.collects,
            shares: entity.shares,
            add_time: entity.add_time,
            visibility_perm: entity.visibility_perm,
            comment_perm: entity.comment_perm,
            danmaku_perm: entity.danmaku_perm,
            download_perm: entity.download_perm,
        }
    }
}

//////// END
