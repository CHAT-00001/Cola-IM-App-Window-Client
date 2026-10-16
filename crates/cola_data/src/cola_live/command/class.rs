// cola_live/cmd/class.rs  -- LIVE - cmd - 分类
// 2026/6/13 07:25

////////

use crate::cola_live::entity::cate::class::LiveClassEntity; // 请根据你实际的 handler 路径微调
use chrono::Utc;
use serde::{Deserialize, Serialize};

////////

// 角色枚举：用于运营/运维/管理员权限控制
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Operator, // 运维人员
    Admin,    // 管理员/运营管理
}

////////

///  # [COMMAND] - 直播流分类 创建命令a```
/// * `desc` 后台管理系统专门使用的分类创建传输对象
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LiveClassCommand {
    #[serde(default)]
    pub id: i64, // 编辑/状态/删除目标 ID
    pub name: String,                // 中文名称 (纠正老系统 i64 错误)
    pub name_en: Option<String>,     // 英文名称 (纠正老系统 i64 错误)
    pub icon: String,                // 分类图标 URL / 样式名
    pub sort: Option<i16>,           // 排序权重 (0-9999)
    pub is_hot: Option<i16>,         // 是否热门: 0.否 1.是
    pub is_recommended: Option<i16>, // 是否推荐: 0.否 1.是
    pub status: Option<i16>,         // 状态: 0禁用 1启用
}

////////

///  # [BUILD] - 构造函数与实体映射
impl LiveClassCommand {
    //

    ////////

    /// # [CASE] - 构造函数：强校验管理员/运维权限，注入操作人进行痕迹溯源，并自动补全所有默认机器字段
    pub fn new(
        role: &UserRole,   // 🔒 权限控制
        operator_uid: i64, // 👁️ 溯源：当前后台操作人员的 UID
        cmd: Self,
    ) -> Result<LiveClassEntity, String> {
        // 1. 🔒 权限硬校验：只有管理员（Admin）或 运维/运营（Operator）才能添加分类
        if role != &UserRole::Admin && role != &UserRole::Operator {
            return Err("权限不足：该操作仅限后台管理或运营人员执行".to_string());
        }

        // 2. 🛡️ 基础验证：分类名称不能为空白字符
        if cmd.name.trim().is_empty() {
            return Err("创建失败：分类中文名称不能为空".to_string());
        }

        // 3. ⏱️ 时间戳准备 (Entity 使用的是 NaiveDateTime，即不带时区的本地/标准时间)
        let now = Utc::now();
        let now_ts = Utc::now().timestamp();

        // 4. 🎯 完美组装并返回符合数据库表结构的干净 Entity
        Ok(LiveClassEntity {
            id: 0, // 自增主键，初始化给 0

            // 👁️ 痕迹溯源：新创建时，创建者和最后修改者都是当前这个 operator_uid
            uid: operator_uid,
            action_uid: operator_uid,

            // 📝 业务核心数据 (强行规避掉了 i64 的坑，保持正常的 String 写入)
            name: cmd.name,
            name_en: cmd.name_en.unwrap_or_default(), // 英文名如果没有，留空字符串
            icon: cmd.icon,

            // 📊 字段默认值智能退避
            sort: cmd.sort.unwrap_or(9999),  // 默认排在最后面 (9999)
            is_hot: cmd.is_hot.unwrap_or(0), // 默认不热门 (0)
            is_recommended: cmd.is_recommended.unwrap_or(0), // 默认不推荐 (0)
            status: 1,                       // 📌 新增加的分类默认严格为 1 (启用)

            // 🤖 机器与人类时间同步
            add_time: now_ts,
            upd_time: now_ts,
            created_at: now,
            updated_at: now,
        })
    }
}

//////// END
