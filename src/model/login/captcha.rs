//! 人机验证响应
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

use bili_core::Data;
use serde::{Deserialize, Serialize};

/// CAPTCHA_URL响应Data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct Captcha {
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
}
