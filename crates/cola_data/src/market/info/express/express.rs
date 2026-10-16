// market/info/express/express.rs
// 市场 - info - 快递 - 快递信息
// 2026/8/3 23:31 Created.

////////

use crate::market::entity::express::express::ExpressEntity;
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 市场 - 快递 - 快递信息
/// * `desc`: 屏蔽底层数据库 Entity 的冗余字段，提供更友好的业务接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressInfo {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub phone: String,
    pub is_enabled: bool,
}

impl ExpressInfo {
    /// # 构造函数
    /// 用于手动创建一个快递信息实例
    pub fn new(id: i64, name: String, code: String, phone: String, is_enabled: bool) -> Self {
        Self {
            id,
            name,
            code,
            phone,
            is_enabled,
        }
    }

    /// # 兜底方法
    /// 当查询不到对应快递时，返回此默认值，避免业务层产生空指针或逻辑异常
    pub fn not_found() -> Self {
        Self {
            id: 0,
            name: "快递不存在".to_string(),
            code: "UNKNOWN".to_string(),
            phone: "000-0000-0000".to_string(),
            is_enabled: false,
        }
    }
}

/// # 转换逻辑
/// 从底层 Entity 转换为业务层 Info
impl From<ExpressEntity> for ExpressInfo {
    fn from(entity: ExpressEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.express_name,
            code: entity.express_code,
            phone: entity.express_phone,
            // 业务层通常更喜欢 bool 类型的状态判定
            is_enabled: entity.express_status == 1,
        }
    }
}

//////// END
