// cola_date/src/gisentity/ban  -- 数据中心 - GIS - entit - 状态
// 2026/4/23 15:55

////////

use chrono::{DateTime, Utc};

////////

/// # [Entity] - 兴趣点 状态 实体表
/// * `pg schema`: `cola_gis`
/// * `table name`: `gis_poi_collect`
pub struct PoiStateEntity {
    pub id: i64,
    /// 形态1：永久封禁标志位
    pub is_banned: bool,
    /// 形态2：限期封禁过期时间
    pub ban_expire_time: Option<DateTime<Utc>>,
}

impl PoiStateEntity {
    /// 综合检查用户是否处于封禁状态
    pub fn is_banned(&self) -> bool {
        // 1. 优先检查永久封禁形态
        if self.is_banned {
            return true;
        }

        // 2. 检查限期封禁形态
        // 如果设定了过期时间，且当前时间还没到，则视为封禁中
        if let Some(expire) = self.ban_expire_time {
            if Utc::now() < expire {
                return true;
            }
        }

        // 3. 两种形态都不满足，则视为正常
        false
    }

    /// 辅助：获取封禁类型描述（可选，方便日志或UI显示）
    pub fn get_ban_type(&self) -> &'static str {
        if self.is_banned {
            "永久封禁"
        } else if self.is_banned() {
            "限期封禁"
        } else {
            "正常"
        }
    }
}

//////// END
