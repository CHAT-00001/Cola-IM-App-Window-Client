// cola_data/src/gisentity/user.rs -- 数据 - GIS - entity - user
// 2026/5/19 21:34

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 短视频创作者表
/// * ``: `PG schema` - PG 模式
/// * `table name`: `video_user` - 表名
#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct VideoUserEntity {
    // ==== 鍩虹韬唤鏍囪瘑 ====
    pub uid: i64,                // 鐢ㄦ埛ID
    pub id: Option<String>,      // 鍙戦€?ID UUID 锛堝鎴风鐢熸垚锛?
    pub show_id: Option<String>, // 鏄剧ず ID
    pub sync_id: Option<String>, // 鍚屾 ID
    pub user_type: i16, // 鐢ㄦ埛绫诲瀷锛?-鏅€氳浼? 2-鍒涗綔鑰? 3-浼佷笟鍙?钃漋) 馃憟 鏂板

    // ==== 鏍稿績璁℃暟鍣紙楂橀鍙樺姩锛屽缓璁厤鍚?Redis 寮傛鍥炲啓锛?====
    pub publish_count: i32,         // 鍙戝竷鐨勮棰戞暟閲?
    pub liked_count: i32,           // 鐐硅禐鐨勮棰戞暟閲忥紙璇ョ敤鎴风偣璧炰簡澶氬皯涓棰戯級
    pub total_favorited_count: i32, // 鑾疯禐鎬绘暟锛堣鐢ㄦ埛鐨勮棰戣鍒汉鐐硅禐鐨勬€绘暟锛?馃憟 鏂板
    pub collected_count: i32, // 鏀惰棌鐨勮棰戞暟閲忥紙淇鍘熷瓧娈靛悕涓?collected_count 淇濇寔璇剰涓€鑷达級
    pub following_count: i32, // 鍏虫敞鐨勪汉鏁?馃憟 鏂板
    pub follower_count: i32,  // 绮変笣鏁伴噺 馃憟 鏂板

    // ==== 绀句氦灞炴€ф墿灞?====
    pub title_at_uids: Option<Vec<i64>>, // 鏍囬@鐨処Ds (涓婚〉甯哥敤甯搁┗@浜虹兢)

    // ==== 鏍稿績鐘舵€佷笌椋庢帶 ====
    pub status: i16,               // 鐘舵€侊細1-姝ｅ父, 2-绂佽█, 3-灏佺 馃憟 鏂板
    pub audit_msg: Option<String>, // 璐﹀彿灏佺/澶勭綒鍘熷洜璇存槑 馃憟 鏂板

    // ==== 缁嗗垎涓氬姟鏉冮檺鎺у埗 ====
    pub publish_perm: Option<String>, // 鍙戝竷鏉冮檺鎺у埗锛堝: 鍏佽鍙戝竷銆佺姝㈠彂甯冦€侀渶瀹℃牳鍚庡彂甯冿級
    pub visite_perm: Option<Vec<i64>>, // 娴忚鏉冮檺鎺у埗锛堥粦鐧藉悕鍗曟帶鍒讹級
    pub comment_perm: i16,            // 璇勮鏉冮檺锛?-鍏佽鎵€鏈変汉, 2-浠呯矇涓? 3-鍏ㄧ綉绂佽█
    pub danmaku_perm: i16,            // 寮瑰箷鏉冮檺锛?-鍏佽, 2-鍏抽棴
    pub collect_perm: i16,            // 鏀惰棌鏉冮檺锛?-鍏佽, 2-绂佹
    pub download_perm: i16,           // 涓嬭浇鏉冮檺锛?-鍏佽浠栦汉涓嬭浇鍘熻棰? 2-绂佹涓嬭浇

    // ==== 鍏煎鎬т笌鏃堕棿鎴?====
    pub addtime: i64,                      // 鍒涘缓鏃堕棿锛堝吋瀹规棫鐗圥HP锛?
    pub sync_at: Option<i64>,              // 鍚屾鏃堕棿
    pub created_at: Option<DateTime<Utc>>, // 鍒涘缓鏃堕棿
    pub updated_at: Option<DateTime<Utc>>, // 鏇存柊鏃堕棿
}

//////// END
