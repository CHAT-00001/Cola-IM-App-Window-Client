// cola_data/src/cola_three/cmd/cola_fs  -- THREE - 服务类型命令
// 2026/7/27

use serde::{Deserialize, Serialize};

/// # [COMMAND] - 服务类型 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreeServerTypeCommand {
    pub code: String, // 代码
    pub name: String, // 中文名
    pub sort: i16,    // 排序
    pub status: i16,  // 1启用 0禁用
}

/// # [COMMAND] - 服务类型 分页查询
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeListQuery {
    pub keyword: Option<String>,
    pub status: Option<i16>,
    pub offset: i64,
    pub limit: i64,
}
