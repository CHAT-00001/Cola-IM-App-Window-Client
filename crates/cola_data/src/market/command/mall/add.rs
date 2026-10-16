// cola_data/src/market/cmd/mall/code
// 数据中心 - MARKET - cmd - 商场 - 发布
// 2026/8/3 22:24 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 创建商场
/// * `desc`: 管理员：新增或修改快递公司
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedMallCommand {
    pub name: String,     // 快递名称
    pub name_en: String,  // 英文名称
    pub phone: String,    // 联系电话
    pub thumb: String,    // 图标路径/链接
    pub code: String,     // 快递代码 (例如: SF, YTO)
    pub sort: i16,        // 排序权重
    pub is_enabled: bool, // 是否启用 (管理员开关)
}

// 构造函数
impl CreatedMallCommand {
    /// # 校验逻辑
    /// 在业务层调用此方法，确保管理员提交的数据符合基本规则
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("快递名称不能为空"));
        }
        if self.code.trim().is_empty() {
            return Err(anyhow::anyhow!("快递代码不能为空"));
        }
        Ok(())
    }
}

//////// END
