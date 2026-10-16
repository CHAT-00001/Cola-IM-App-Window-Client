// cola_dynamic/shut.rs  -- 直播间 禁言 命令
// 2026/6/13 09:33

////////

use crate::cola_live::entity::shut::LiveStreamShutEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

// 角色枚举：允许主播、房管和管理员执行禁言
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Operator, // 房管 / 主播本人
    Admin,    // 平台超级管理员
}

////////

/// # [COMMAND] - 直播流禁言 创建命令
/// * `desc` 直播间前端或管理后台执行禁言/踢人操作的传输对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStreamShutCommand {
    pub live_id: i64,   // 直播间ID
    pub record_id: i64, // 直播场次记录ID
    pub user_id: i64,   // 被禁言的目标用户ID
    pub duration: i64,  // 禁言时长(秒)
}

////////

/// # [BUILD] - 构造函数与实体映射
impl LiveStreamShutCommand {
    /// 构造函数：校验操作者权限，自动生成双重时间戳，输出干净的禁言实体
    pub fn new(
        role: &UserRole,
        operator_uid: i64,
        cmd: Self,
    ) -> Result<LiveStreamShutEntity, String> {
        // 1. 权限硬校验：普通观众（User）不能执行禁言
        if role != &UserRole::Admin && role != &UserRole::Operator {
            return Err("权限不足：该操作仅限主播、房管或管理员执行".to_string());
        }

        // 2. 基础验证：不能禁言自己，且时长必须合法
        if operator_uid == cmd.user_id {
            return Err("操作失败：不能对自己执行禁言操作".to_string());
        }
        if cmd.duration <= 0 {
            return Err("操作失败：禁言时长必须大于0秒".to_string());
        }

        let now = Utc::now().naive_utc();
        let now_ts = Utc::now().timestamp() as i32;

        // 3. 完美转换输出 Entity
        Ok(LiveStreamShutEntity {
            id: 0, // 自增主键，初始化给 0
            operator_uid,
            live_id: cmd.live_id,
            record_id: cmd.record_id,
            user_id: cmd.user_id,
            duration: cmd.duration,
            status: 1, // 默认直接有效生效 (1)
            add_time: now_ts,
            upd_time: now_ts,
            created_at: now,
            updated_at: now,
        })
    }
}

//////// END
