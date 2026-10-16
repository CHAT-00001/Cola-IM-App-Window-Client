// infra/src/api/auth//login.rs -- INFRA - API - AUTH - 验证码
// 2026/10/16 06:35 Created.

////////

use crate::model::vo::auth::CaptchaResponse;
use cola_data::app::data::AppData;
use cola_data::app::error;

////////

const BASC_URL: &str = "http://127.0.0.1:8080/api/v2/auth"; // 测试地址

////////

/// # [LOGGER API] - 验证码接口
pub struct LoginApi;

// 构造函数
impl LoginApi {
    //

    ////////

    /// # 1. [API HANDLER] - 手机验证码登录
    pub(crate) async fn login_by_phone_code(
        area: &str,
        phone_no: &str,
    ) -> AppData<CaptchaResponse> {
        //

        ////////

        // 💡 - CHECK - 验证手机号合法性
        if let Err(e) = verify_phone_no(area, phone_no) {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("[] - 请输入正确的手机号: {:?}", e),
                None,
            );
        }

        info! {"正在登录..."}

        // 💡 - 发送HTTP请求获取/下发验证码
        match fetch_send_captcha(area, phone_no).await {
            Ok(resp) => AppData::ok(resp).with_msg("验证码已下发"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("获取验证码失败: {:?}", e), None),
        }
    }

    ////////
}
//////// END
