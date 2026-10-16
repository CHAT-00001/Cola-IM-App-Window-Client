// cola_data/src/common/kits/snow.rs -- 数据 - 公共 - 工具包 - 雪花ID
// 2026/9/11 07:53 Created.

////////

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// 自定义纪元时间 (2024-01-01 00:00:00 UTC 的毫秒数，保证时间戳在范围内不会溢出)
const EPOCH: u64 = 1704067200000;

const NODE_ID_BITS: u32 = 10; // 节点 ID 占 10 位 (最大支持 1024 个节点)
const SEQUENCE_BITS: u32 = 12; // 序列号占 12 位 (每毫秒最多支持 4096 个ID)

const MAX_NODE_ID: u64 = (1 << NODE_ID_BITS) - 1;
const MAX_SEQUENCE: u32 = (1 << SEQUENCE_BITS) - 1;

const NODE_ID_SHIFT: u32 = SEQUENCE_BITS;
const TIMESTAMP_SHIFT: u32 = SEQUENCE_BITS + NODE_ID_BITS;

////////

/// # [BUILD] - 生成标准分布式雪花ID，返回 i64（适配数据库 BIGINT）
/// * `node_id`: 0~1023，多实例部署时每个节点分配不同编号，防止分布式冲突
pub fn next_id(node_id: u16) -> i64 {
    let node_id = (node_id as u64) & MAX_NODE_ID;

    // 获取当前毫秒级时间戳
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    if now < EPOCH {
        panic!("系统时间异常：当前时间小于设定纪元！");
    }

    let timestamp = now - EPOCH;

    // 原子自增序列号
    static SEQUENCE: AtomicU32 = AtomicU32::new(0);
    let mut seq = SEQUENCE.fetch_add(1, Ordering::Relaxed);

    // 毫秒内序列号超限后归零
    if seq > MAX_SEQUENCE {
        SEQUENCE.store(0, Ordering::Relaxed);
        seq = 0;
    }

    // 🚀 位运算拼装 64 位标准雪花 ID
    // 结构：[ 1位符号位(0正) ] [ 41位时间戳 ] [ 10位节点ID ] [ 12位序列号 ]
    let id = ((timestamp as i64) << TIMESTAMP_SHIFT as i64)
        | ((node_id as i64) << NODE_ID_SHIFT)
        | (seq as i64);

    id
}

////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snow_i64() {
        let id = next_id(1);
        println!("snow id(i64) = {}", id);
        assert!(id > 0);
    }
}

//////// END
