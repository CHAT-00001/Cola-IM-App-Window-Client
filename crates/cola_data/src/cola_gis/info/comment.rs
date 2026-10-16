// cola_data/src/cola_gis/info/add  -- 数据中心 - GIS - info - 兴趣点评论
// 2026/5/21 00:58

////////

use crate::cola_gis::entity::comment::PoiCommentEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 兴趣点 评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoiCommentInfo {
    pub id: i64,                // 评论 ID
    pub user_id: i64,           // 用户 ID
    pub poi_id: i64,            // 兴趣点 ID
    pub parent_id: Option<i64>, // 父评论 ID
    pub content: String,        // 内容
    pub likes: i32,             // 点赞数量
    pub reply: i32,             // 回复数量
    pub send_time: i64,         // 添加时间 - 机器
    pub sync_time: i64,         // 同步时间 - 机器
}

/// # 构造
impl PoiCommentInfo {
    ////////

    /// # [BUILD] - 新建
    pub fn new(
        id: i64,
        user_id: i64,
        poi_id: i64,
        parent_id: Option<i64>,
        content: String,
        likes: i32,
        reply: i32,
        send_time: i64,
        sync_time: i64,
        video_author_id: i64, // 浼犲叆瑙嗛浣滆€呯殑 UID
    ) -> Self {
        // 濡傛灉璇勮鐨勫彂甯冭€?ID 绛変簬 瑙嗛浣滆€呯殑 ID锛屽垯涓?true
        let is_author = user_id == video_author_id;

        Self {
            id,
            user_id,
            poi_id,
            parent_id,
            content,
            likes,
            reply,
            send_time,
            sync_time,
        }
    }

    /// # [FROM] - 转换
    pub fn from_entity(entity: PoiCommentEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            poi_id: entity.poi_id,
            parent_id: entity.parent_id,
            content: entity.content,
            likes: entity.likes,
            reply: entity.reply,
            send_time: entity.send_time,
            sync_time: entity.sync_time,
        }
    }
}

//////// END
