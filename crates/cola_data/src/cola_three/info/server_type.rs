// cola_data/src/cola_three/info/cola_fs  -- THREE - 类型信息
// 2026/6/18

////////

use crate::cola_three::entity::server_type::ThreeServerTypeEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 服务分类信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerTypeInfo {
    pub id: i64,      // ID
    pub uid: i64,     // 作者ID
    pub code: String, // 代码
    pub name: String, // 名称
    pub sort: i16,    // 排序
    pub status: i16,  // 状态
}

// 构造实现

impl From<ThreeServerTypeEntity> for ServerTypeInfo {
    // # [CASE] - 实体转换
    fn from(e: ThreeServerTypeEntity) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            code: e.code,
            name: e.name,
            sort: e.sort,
            status: e.status,
        }
    }
}

//////// END
