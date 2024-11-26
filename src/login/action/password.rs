//！密码账号登录
//！```
//! #密码登录流程
//!
//! ```

#![allow(dead_code)]
use std::fmt::Display;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};

use super::captcha::CaptchaData;
use crate::{common::Query, error::Error};

/// 获取公钥&盐(只有web端需要)
pub const LOGIN_KEY_URL: &str = "https://passport.bilibili.com/x/passport-login/web/key";

/// 登录盐
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginKeyData {
    /// 有效时间为 20s
    #[serde(rename = "hash")]
    salt: String,
    key: String,
}
impl LoginKeyData {
    pub fn take(self) -> (String, String) {
        (self.salt, self.key)
    }
}
/// 登录操作(post)
pub const WEB_LOGIN_URL: &str = "https://passport.bilibili.com/x/passport-login/web/login";

pub fn decode_password<S>(password: S, loginkey: LoginKeyData) -> Result<String,Error>
where
    S: AsRef<str> + Display,
{
    let (salt, key) = loginkey.take();

    let data = format!("{}{}", salt, password);

    let mut rng = rand::thread_rng();
    let pub_key = RsaPublicKey::from_public_key_pem(&key)?;
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())?;
    let password = URL_SAFE.encode(enc_data);
    Ok(password)
}
/// 登录查询头
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginQuery {
    username: String,
    password: String,
    keep: u8,
    #[serde(rename = "token")]
    captcha_token: String,
    #[serde(rename = "challenge")]
    captcha_challenge: String,
    validate: String,
    seccode: String,
    go_url: Option<String>,
    source: Option<String>,
}
impl WebLoginQuery {
    pub fn new(
        username: String,
        password: String,
        captcha: CaptchaData,
        validate: String,
        go_url: Option<String>,
        source: Option<String>,
    ) -> Self {
        let (captcha_token, captcha_challenge) = captcha.take();
        let seccode = format!("{}|jordan", validate);

        WebLoginQuery {
            username,
            password,
            keep: 0,
            captcha_token,
            captcha_challenge,
            validate,
            seccode,
            go_url,
            source,
        }
    }
}

impl Query for WebLoginQuery {}

/// 登录响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginData {
    pub message: String,
    pub refresh_token: String,
    pub timestamp: u64,
    pub url:String,
}


#[cfg(feature = "session")]
mod session {
    use crate::{
        common::{Query, Response, Session},
        error::Error,
        login::action::{LoginData, WebLoginResponse},
    };

    use super::{LoginKeyData, WebLoginData, WebLoginQuery, LOGIN_KEY_URL, LOGIN_URL};

    impl Session {
        /// 获取登录秘钥
        pub async fn get_login_key(&self) -> Result<LoginKeyData, Error> {
            let url = LOGIN_KEY_URL;
            let logindata = self
                .get(url)
                .send()
                .await?
                .json::<WebLoginResponse>()
                .await?
                .result()?;

            if let LoginData::LoginKeyData(data) = logindata {
                Ok(data)
            } else {
                Err(Error::from("Unexpected response type"))
            }
        }

        /// web端登录
        pub async fn web_login(&self, query: WebLoginQuery) -> Result<WebLoginData, Error> {
            let url = format!("{}?{}", WEB_LOGIN_URL, query.to_query()?);
            let logindata = self
                .post(url)
                .send()
                .await?
                .json::<WebLoginResponse>()
                .await?
                .result()?;

            if let LoginData::WebLoginData(data) = logindata {
                Ok(data)
            } else {
                Err(Error::from("Unexpected response type"))
            }
        }
    }
}
