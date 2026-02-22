//! 登陆的请求
//!
//! 先完成人机验证拿到validate与seccode
//!
//! 请求LoginKeyQuery拿到盐,对密码进行加密
//!
//! LoginQuery登陆(post)

use base64::{Engine, prelude::BASE64_URL_SAFE};
use bili_core::{Data, ToQuery};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey, pkcs8::DecodePublicKey};
use serde::{Deserialize, Serialize};

use crate::{captcha::Captcha, error::Error};

/// 获取公钥&盐(只有web端需要)
pub const LOGIN_KEY_URL: &str = "https://passport.bilibili.com/x/passport-login/web/key";

/// 获取公钥&盐(只有web端需要)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToQuery)]
pub struct LoginKeyQuery;
impl LoginKeyQuery {
    pub fn new() -> Self {
        Self
    }
}

/// Web端登录操作(post)
pub const LOGIN_URL: &str = "https://passport.bilibili.com/x/passport-login/web/login";
pub const PASSWORD_LOGIN_URL: &str = LOGIN_URL;

/// 密码登录操作(post)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ToQuery)]
pub struct PasswordLoginQuery {
    pub username: String,
    pub password: String,
    // 保留价值存疑
    pub keep: u8,
    pub token: String,
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
    //下面这两个没什么实际意义
    // pub go_url: Option<String>,
    // pub source: Option<String>,
}
impl PasswordLoginQuery {
    pub fn new(
        username: String,
        password: String,
        captcha: Captcha,
        validate: String,
        // go_url: Option<String>,
        // source: Option<String>,
    ) -> Self {
        let Captcha { token, geetest, .. } = captcha;
        let challenge = geetest.challenge;
        let seccode = format!("{}|jordan", validate);

        PasswordLoginQuery {
            username,
            password,
            keep: 0,
            token,
            challenge,
            validate,
            seccode,
            // go_url,
            // source,
        }
    }
}

/// 登录盐
///
/// 有效时间为 20s
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct LoginKey {
    #[serde(rename = "hash")]
    pub salt: String,
    pub key: String,
}

/// 密码加密
impl LoginKey {
    pub fn decode_password(&self, password: &str) -> Result<String, Error> {
        let data = format!("{}{}", self.salt, password);

        let mut rng = rand::thread_rng();
        let password = RsaPublicKey::from_public_key_pem(&self.key)?
            .encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())
            .map(|key| BASE64_URL_SAFE.encode(key))?;

        Ok(password)
    }
}

/// 登录响应数据
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct PasswordLogin {
    pub message: String,
    pub refresh_token: String,
    pub timestamp: u64,
    pub url: String,
}
