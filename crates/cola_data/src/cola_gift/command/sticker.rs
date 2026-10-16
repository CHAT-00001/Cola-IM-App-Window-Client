// cola_data/src/gift/cmd/score.rs  -- 数据中心 - GIFT - cmd - 自定义表情贴纸
// 2026/7/8 11:54

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [COMMAND] - 表情包贴纸 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GiftStickerCommand {
    pub id: i64,          // ID
    pub user_id: i64,     // 用户 ID
    pub classify_id: i64, // 分类 ID
    pub name: String,     // 中文名称
    pub name_en: String,  // 英文名称
    pub url: String,      // 地址
    pub sort: i16,        // 排序
    pub status: i16,      // 状态
    pub add_time: i64,    // 添加时间 - 机器
    pub upd_time: i64,    // 更新时间 - 机器
}

// 构造实现
impl GiftStickerCommand {
    // 1. ✅️ new 新建

    // 2. ❌️ empty 空

    // 3. ❗️ from 转换
}

//////// END
