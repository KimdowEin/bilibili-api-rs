//! 人机验证
//! 人机验证流程
//! 1.请求验证码参数，得到登录密钥key与极验idgt和极验KEYchallenge
//! 2.进行滑动or点击验证(feature=manual 提供了一个网站跳转函数用来手动验证)
//! 3.返回验证结果validate与seccode，进行短信或密码登录


#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// 申请captcha验证码
pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";

/// CAPTCHA_URL响应Data
#[derive(Debug, Serialize, Deserialize)]
pub struct CaptchaData {
    token: String,
    geetest: Geetest,
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

// #[cfg(feature = "session")]
// mod session {
//     use crate::{
//         common::{Response, Session},
//         error::Error,
//         login::action::{LoginData, WebLoginResponse},
//     };

//     use super::{CaptchaData, CAPTCHA_URL};

//     impl Session {
//         /// 获取人机验证
//         pub async fn captcha(&self) -> Result<CaptchaData, Error> {
//             let logindata = self
//                 .get(CAPTCHA_URL)
//                 .send()
//                 .await?
//                 .json::<WebLoginResponse>()
//                 .await?
//                 .result()?;

//             if let LoginData::CaptchaData(data) = logindata {
//                 Ok(data)
//             } else {
//                 Err(Error::from("Unexpected response type"))
//             }
//         }
//     }
// }

/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
pub fn manual_verification(geetest: &Geetest) -> Result<(), std::io::Error> {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    webbrowser::open(&url)
}

