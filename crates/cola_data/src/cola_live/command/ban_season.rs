// cola_dynamic/ban_season.rs  -- 直播流主播封禁原因创建命令
// 2026/6/13 08:15

////////

use crate::cola_live::entity::anchor::ban_reason::LiveStreamAnchorBanReasonEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

// 角色枚举：用于运营/运维/管理员权限控制
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Operator,
    Admin, // 管理员/风控主管 (MGC权限)
}

////////

/// # [COMMAND] - 直播流主播封禁原因 创建命令
/// * `desc` 风控后台管理系统添加违规封禁类型的传输对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveAnchorBanReasonCommand {
    pub icon: String,           // 图标 URL 或 样式名
    pub name: String,           // 违规中文名称
    pub name_en: String,        // 违规英文名称
    pub description: String,    // 中文详细描述
    pub description_en: String, // 英文详细描述
    pub remark: Option<String>, // 备注说明/签注
}

////////

/// # [BUILD] - 构造函数与实体映射
impl LiveAnchorBanReasonCommand {
    /// 构造函数：硬校验管理员(MGC)权限，自动记录操作人UID实现审计溯源，补全机器时间
    pub fn new(
        role: &UserRole,
        operator_uid: i64,
        cmd: Self,
    ) -> Result<LiveStreamAnchorBanReasonEntity, String> {
        // 1. 权限硬校验：风控敏感操作，必须是最高权限管理员
        if role != &UserRole::Admin {
            return Err("权限不足：该操作仅限平台系统管理员或风控主管执行".to_string());
        }

        // 2. 基础验证：违规名称和描述不能为空
        if cmd.name.trim().is_empty() || cmd.description.trim().is_empty() {
            return Err("创建失败：违规名称和详细描述不能为空".to_string());
        }

        let now = Utc::now();
        let now_ts = now.timestamp() as i32;

        // 3. 完美转换输出 Entity
        Ok(LiveStreamAnchorBanReasonEntity {
            id: 0, // 自增主键，初始化给 0
            operator_uid,
            icon: cmd.icon,
            name: cmd.name,
            name_en: cmd.name_en,
            description: cmd.description,
            description_en: cmd.description_en,
            remark: cmd.remark.unwrap_or_default(),
            status: 1, // 默认直接启用生效 (1)
            add_time: now_ts,
            upd_time: now_ts,
            created_at: now,
            updated_at: now,
        })
    }
}

//////// END
