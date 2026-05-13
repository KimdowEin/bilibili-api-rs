//! 人机验证请求
//!
//! 扫码登陆不需要人机验证
//!
//! 人机验证流程
//!
//! 1.请求验证码，返回Captcha,拿到gt和challenge
//!
//! 2.进行滑动or点击验证(feature=manual 提供了一个网站用来手动验证)
//!
//! 3.返回验证结果validate与seccode，进行后续短信或密码登录

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

/// 申请captcha验证码
pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";

/// 申请captcha验证码
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToQuery)]
pub struct CaptchaQuery;

impl CaptchaQuery {
    pub fn new() -> Self {
        Self
    }
}

/// CAPTCHA_URL响应Data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct Captcha {
    #[serde(rename = "type")]
    pub captcha_type: String,
    pub token: String,
    pub geetest: Geetest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geetest {
    pub challenge: String,
    pub gt: String,
}

#[cfg(test)]
mod tests {
    use bili_core::BiliResponse;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_captcha() {
        let session = bili_test_utils::session_from_state(Default::default());

        let url = CaptchaQuery::new().to_query().unwrap().to_url(CAPTCHA_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/captcha.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_captcha() {
        let json = include_str!("../../tests/datas/captcha.json");
        serde_json::from_str::<BiliResponse<Captcha>>(json).unwrap();
    }
}
