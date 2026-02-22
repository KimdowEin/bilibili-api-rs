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
    use std::sync::Arc;

    use bili_core::BiliResponse;
    use bili_service::{Session, SessionState};
    use reqwest::ClientBuilder;

    use super::*;

    #[test]
    fn test_deserialize_captcha() {
        let json = r#"{
            "type":"geetest",
            "token":"92de00ec444b4f27bd5ab92843663c47",
            "geetest":{
                "challenge":"48f520597a9b91bfb7f322fc32629b11",
                "gt":"ac597a4506fee079629df5d8b66dd4fe"
            },
            "tencent":{
                "appid":""
            }
        }"#;

        serde_json::from_str::<Captcha>(json).unwrap();
    }

    #[tokio::test]
    async fn test_get_captcha() {
        let state = SessionState::default();

        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .build()
            .unwrap();
        let session = Session::new(client, Arc::new(state));

        let url = CaptchaQuery::new().to_query().unwrap().to_url(CAPTCHA_URL);

        let captcha = session
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<BiliResponse<Captcha>>()
            .await
            .unwrap()
            .data()
            .unwrap();

        assert_eq!(captcha.captcha_type, "geetest")
    }
}
