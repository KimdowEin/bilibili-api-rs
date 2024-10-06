#![allow(dead_code)]

use crate::common::Query;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;

pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";
pub const LOGIN_KEY_URL: &str = "https://passport.bilibili.com/x/passport-login/web/key";
pub const LOGIN_URL: &str = "https://passport.bilibili.com/x/passport-login/web/login";
/******人类行为验证******/

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

/******账号密码登录******/

/// 登录盐
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginKeyData {
    #[serde(rename = "hash")]
    salt: String,
    key: String,
}

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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebLoginData {
    code: WebLoginCode,
}
impl WebLoginData {
    pub fn code(&self) -> WebLoginCode {
        self.code.clone()
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i64)]
pub enum WebLoginCode {
    Success = 0,
    CaptchaError = -105,
    RequestError = -400,
    PasswordError = -629,
    UsernameEmpty = -653,
    SubmitTimeout = -662,
    MissingParams = -2001,
    NeedPhoneOrEmail = -2100,
    LoginKeyError = 2400,
    GeetestError = 2406,
    RsaDecryptFail = 86000,
    Unknown,
}
impl Display for WebLoginCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "成功"),
            Self::CaptchaError => write!(f, "验证码错误"),
            Self::RequestError => write!(f, "请求错误"),
            Self::PasswordError => write!(f, "账号或密码错误"),
            Self::UsernameEmpty => write!(f, "用户名或密码不能为空"),
            Self::SubmitTimeout => write!(f, "提交超时"),
            Self::MissingParams => write!(f, "缺少必要的参数"),
            Self::NeedPhoneOrEmail => write!(f, "需验证手机号或邮箱"),
            Self::LoginKeyError => write!(f, "登录秘钥错误"),
            Self::GeetestError => write!(f, "极验服务出错"),
            Self::RsaDecryptFail => write!(f, "RSA解密失败"),
            Self::Unknown => write!(f, "未知错误"),
        }
    }
}


#[cfg(feature = "session")]
mod session {
    use super::*;
    use crate::common::Session;
    use crate::common::{Data, ResponseData};
    use reqwest::Error;

    impl Session {
        /// 获取人机验证
        pub async fn captcha(&self) -> Result<CaptchaData, Error> {
            let url = CAPTCHA_URL;
            let response = self
                .get(url)
                .send()
                .await?
                .json::<ResponseData>()
                .await?
                .take();

            if let Some(Data::CaptchaData(captcha_data)) = response {
                Ok(captcha_data)
            } else {
                panic!("Unexpected response type")
            }
        }

        /// 获取登录秘钥
        pub async fn get_login_key(&self) -> Result<LoginKeyData, Error> {
            let url = LOGIN_KEY_URL;
            let response = self
                .get(url)
                .send()
                .await?
                .json::<ResponseData>()
                .await?
                .take();

            if let Some(Data::LoginKey(login_key)) = response {
                Ok(login_key)
            } else {
                panic!("Unexpected response type")
            }
        }
        /// web端登录
        pub async fn web_login(&self, query: String) -> Result<WebLoginData, Error> {
            let url = format!("{}?{}", LOGIN_URL, query);
            let response = self.post(url).send().await?.json::<WebLoginData>().await?;
            Ok(response)
        }
    }
}
#[cfg(feature = "session")]
pub use session::*;

/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
pub fn manual_verification(geetest: &Geetest) {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Error opening browser: {}", e);
    }
}
