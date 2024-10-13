#![allow(dead_code)]
////////////////
/// 人机验证 ///
////////////////
use serde::{Deserialize, Serialize};

pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";

/// 人类验证响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CaptchaData {
    pub token: String,
    pub geetest: Geetest,
}
impl CaptchaData {
    /// 取出登录所需的特征码
    pub fn take(self) -> (String, String) {
        (self.token, self.geetest.challenge)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geetest {
    pub challenge: String,
    pub gt: String,
}

#[cfg(feature = "session")]
mod session {
    use crate::{
        common::{Response, Session},
        error::Error,
        login::action::{LoginData, WebLoginResponse},
    };

    use super::{CaptchaData, CAPTCHA_URL};

    impl Session {
        /// 获取人机验证
        pub async fn captcha(&self) -> Result<CaptchaData, Error> {
            let logindata = self
                .get(CAPTCHA_URL)
                .send()
                .await?
                .json::<WebLoginResponse>()
                .await?
                .result()?;

            if let LoginData::CaptchaData(data) = logindata {
                Ok(data)
            } else {
                Err(Error::from("Unexpected response type"))
            }
        }
    }
}

/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
pub fn manual_verification(geetest: &Geetest) -> Result<(), std::io::Error> {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    webbrowser::open(&url)
}
