//！密码账号登录
//！```
//! #密码登录流程
//!
//! ```

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::{model::login::captcha::Captcha, traits::Query};


/// 获取公钥&盐(只有web端需要)
pub const LOGIN_KEY_URL: &str = "https://passport.bilibili.com/x/passport-login/web/key";

/// Web端登录操作(post)
pub const LOGIN_URL: &str = "https://passport.bilibili.com/x/passport-login/web/login";


/// 获取公钥&盐(只有web端需要)
#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LoginKeyQuery;
impl Query for LoginKeyQuery {}

/// 登录操作(post)
#[derive(Debug,Default, Serialize, Deserialize)]
pub struct LoginQuery {
    pub username: String,
    pub password: String,
    pub keep: u8,
    pub token: String,
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
    pub go_url: Option<String>,
    pub source: Option<String>,
}
impl LoginQuery {
    pub fn new(
        username: String,
        password: String,
        captcha: Captcha,
        validate: String,
        go_url: Option<String>,
        source: Option<String>,
    ) -> Self {
        let Captcha{
            token,
            geetest,
        } = captcha;
        let challenge = geetest.challenge;
        let seccode = format!("{}|jordan", validate);

        LoginQuery {
            username,
            password,
            keep: 0,
            token,
            challenge,
            validate,
            seccode,
            go_url,
            source,
        }
    }
}
impl Query for LoginQuery {}

