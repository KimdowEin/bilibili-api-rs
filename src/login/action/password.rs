#![allow(dead_code)]
///////////////////
/// 账号密码登录 ///
///////////////////
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use super::captcha::CaptchaData;
use crate::common::Query;

/// 获取公钥&盐(web端)
pub const LOGIN_KEY_URL: &str = "https://passport.bilibili.com/x/passport-login/web/key";

/// 登录盐
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginKeyData {
    /// 有效时间为 20s
    #[serde(rename = "hash")]
    pub salt: String,
    pub key: String,
}

pub const LOGIN_URL: &str = "https://passport.bilibili.com/x/passport-login/web/login";

/// 登录查询头
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginQuery {
    username: String,
    password: String,
    keep: u8,
    token: String,
    challenge: String,
    validate: String,
    seccode: String,
}
impl WebLoginQuery {
    pub fn new(
        loginkey: LoginKeyData,
        username: String,
        password: String,
        captcha_data: CaptchaData,
        validate: String,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let data = format!("{}{}", loginkey.salt, password);
        let data = data.as_bytes();
        let pub_key = loginkey.key;
        let pub_key = RsaPublicKey::from_public_key_pem(&pub_key).unwrap();
        let enc_data = pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
            .expect("failed to encrypt");
        let password = URL_SAFE.encode(enc_data);
        let (token, challenge) = captcha_data.take();
        let seccode = format!("{}|jordan", validate);
        WebLoginQuery {
            username,
            password,
            keep: 0,
            token,
            challenge,
            validate,
            seccode,
        }
    }
}

impl Query for WebLoginQuery {}

/// 登录响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginData {
    message: String,
    refresh_token: String,
    timestamp: u64,
}



#[cfg(feature = "session")]
mod session{
    use crate::{common::{Query, Response, Session}, error::Error, login::action::{LoginData, WebLoginResponse}};

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
            }else{
                Err(Error::from("Unexpected response type"))
            }
        }

        /// web端登录
        pub async fn web_login(&self, query: WebLoginQuery) -> Result<WebLoginData, Error> {
            let url = format!("{}?{}", LOGIN_URL, query.to_query()?);
            let logindata = self
                .post(url)
                .send()
                .await?
                .json::<WebLoginResponse>()
                .await?
                .result()?;

            if let LoginData::WebLoginData(data) = logindata {
                Ok(data)
            }else{
                Err(Error::from("Unexpected response type"))
            }
        }
        
        
    }
}
