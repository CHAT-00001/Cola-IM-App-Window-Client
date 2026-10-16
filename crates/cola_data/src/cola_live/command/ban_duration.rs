// cola_dynamic/ban_duration.rs  -- 直播封禁时长创建命令
// 2026/6/13 08:28

////////

use crate::cola_live::entity::anchor::ban_duration::LiveStreamBanDurationEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

// 角色枚举：用于运营/运维/管理员权限控制
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Operator, // 风控审核员
    Admin,    // 高级管理员/风控主管 (可配置高级权限)
}

////////

/// # [COMMAND] - 直播封禁时长 创建命令
/// * `desc` 风控后台管理系统添加封禁时长阶梯的选择项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveBanDurationCommand {
    pub name: String,          // 中文名称 (例如: "10分钟")
    pub name_en: String,       // 英文名称 (例如: "10m")
    pub duration: i64,         // 时长(秒)
    pub is_admin: Option<i16>, // 是否管理员专属(默认0)
    pub sort: Option<i16>,     // 排序(默认9999)
}

////////

/// # [BUILD] - 构造函数与实体映射
impl LiveBanDurationCommand {
    /// 构造函数：校验管理员权限，并自动计算时长权限阈值与机器时间
    pub fn new(role: &UserRole, cmd: Self) -> Result<LiveStreamBanDurationEntity, String> {
        // 1. 权限硬校验：配置系统核心字典表，必须是管理员
        if role != &UserRole::Admin {
            return Err("权限不足：只有系统管理员可以配置封禁时长字典表".to_string());
        }

        // 2. 基础验证：名称和时长不能不合法
        if cmd.name.trim().is_empty() || cmd.duration <= 0 {
            return Err("创建失败：名称不能为空且时长必须大于0秒".to_string());
        }

        // 3. 业务逻辑智能退避：如果用户没传 is_admin，我们自动判断。
        // 根据你的业务注释：超过 180天（180 * 24 * 3600 = 15,552,000秒）自动判定为管理员专属选项
        let auto_is_admin = if cmd.duration >= 15552000 { 1 } else { 0 };
        let final_is_admin = cmd.is_admin.unwrap_or(auto_is_admin);

        let now = Utc::now();
        let now_ts = now.timestamp() as i32;

        // 4. 返回干净透明的 Entity
        Ok(LiveStreamBanDurationEntity {
            id: 0, // 自增主键，初始化给 0
            name: cmd.name,
            name_en: cmd.name_en,
            duration: cmd.duration,
            is_admin: final_is_admin,
            sort: cmd.sort.unwrap_or(9999),
            status: 1, // 默认直接生效 (1)
            add_time: now_ts,
            upd_time: now_ts,
            created_at: now,
            updated_at: now,
        })
    }
}

//////// END
