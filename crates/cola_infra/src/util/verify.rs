// util/verify.rs -- 验证工具包
// 2026/10/16 06:26 Created.

////////

use crate::error::{Result, SdkError};

////////

/// # [UTIL] - 验证手机号
/// * `desc`: `校验区号与手机号格式, 区号需为 + 开头的 1-4 位数字, 手机号为 5-20 位数字`
pub fn verify_phone_no(
    area: &str,  // 区号 "+86"
    phone: &str, // 电话号码
) -> Result<()> {
    let area_digits = area.strip_prefix('+').unwrap_or("");
    if area_digits.is_empty()
        || area_digits.len() > 4
        || !area_digits.chars().all(|c| c.is_ascii_digit())
    {
        return Err(SdkError::InvalidContext(format!("invalid area code: {area}")));
    }
    if !(5..=20).contains(&phone.len()) || !phone.chars().all(|c| c.is_ascii_digit()) {
        return Err(SdkError::InvalidContext(format!("invalid phone number: {phone}")));
    }
    Ok(())
}

//////// END