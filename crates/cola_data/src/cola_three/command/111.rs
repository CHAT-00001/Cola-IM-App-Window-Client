// cola_data/src/cola_three/cmd/111  -- 第三方服务分类 命令
// 2026/7/27

use serde::{Deserialize, Serialize};

/// # [COMMAND] - 第三方服务分类命令
/// 用于新增/编辑服务分类
/// 对齐 ThreeCategoryEntity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreeCategoryCommand {
    pub code: String,            // 代码
    pub name: String,            // 英文名称
    pub name_zh: String,         // 中文名
    pub remark: Option<String>,  // 备注
    pub sort: i16,               // 排序: 默认9999
    pub status: i16,             // 1启用 0禁用
    pub owner: i16,              // 所有权: 0私有 1第三方
}