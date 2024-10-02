#![allow(dead_code)]

use crate::session::{Data, Query, ResponseData, Session};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use reqwest::Error;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;

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

impl Session {
    /// 获取人机验证
    pub async fn captcha(&self) -> Result<CaptchaData, Error> {
        let url = "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";
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
}
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
#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub struct WebLoginData {
    code: WebLoginCode,
}

#[derive(Debug, Serialize_repr, Deserialize_repr,PartialEq)]
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

impl Session {
    /// 获取登录秘钥
    pub async fn get_login_key(&self) -> Result<LoginKeyData, Error> {
        let url = "https://passport.bilibili.com/x/passport-login/web/key";
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
        let url = format!(
            "{}?{}",
            "https://passport.bilibili.com/x/passport-login/web/login", query
        );
        let response = self.post(url).send().await?.json::<WebLoginData>().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "manual")]
    #[tokio::test]
    /// 这个要单独测试，因为需要手动输入验证码
    /// 顺便存一下key，其他测试就不用反复登录了
    async fn test_web_login() {
        use std::{
            fs::File,
            io::{BufRead, Read},
            thread,
            time::Duration,
        };
        let mut session = Session::new();
        let captcha_data = session.captcha().await.unwrap();

        // 耗时操作，发送心跳保持连接
        manual_verification(&captcha_data.geetest);
        thread::sleep(Duration::from_secs(10));
        session.heartbeat().await.unwrap();

        let stdin = std::io::stdin();
        let mut validate = String::new();
        stdin.lock().read_line(&mut validate).unwrap();
        let validate = validate.trim().to_owned();

        let mut buf = String::new();
        File::open("test/user.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        let buf = buf.lines().collect::<Vec<&str>>();
        let username = buf[0].to_owned();
        let password = buf[1].to_owned();

        let loginkey = session.get_login_key().await.unwrap();
        let query = WebLoginQuery::new(loginkey, username, password, captcha_data, validate)
            .to_query()
            .unwrap();
        let url = format!(
            "{}?{}",
            "https://passport.bilibili.com/x/passport-login/web/login", query
        );
        let response = session.post(url).send().await.unwrap().json::<WebLoginData>().await.unwrap();
        // let response = session.web_login(query).await.unwrap();
        assert_eq!(WebLoginCode::Success, response.code);

        // 存key
        session.mixin_key().await.unwrap();
        let key = session.key();
        let _ = std::fs::write("test/key.txt", key);
    }

    #[tokio::test]
    async fn test_get_login_key() {
        let session = Session::new();
        let loginkey = session.get_login_key().await.unwrap();
        println!("loginkey:{:?}", loginkey);
    }
}
