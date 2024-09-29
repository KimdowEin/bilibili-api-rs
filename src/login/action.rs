#![allow(dead_code)]
use core::panic;
use std::fmt::Display;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::session::{Data, Query, ResponseData, Session};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};


/******人类行为验证******/

/// 人类验证响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CaptchaData {
    pub token: String,
    pub geetest: Geetest,
}
impl CaptchaData {
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
    pub async fn captcha(&self) -> CaptchaData {
        let url = "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";
        let response = self
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<ResponseData>()
            .await
            .unwrap()
            .take()
            .unwrap();
        match response {
            Data::CaptchaData(captcha_data) => captcha_data,
            _ => panic!("Unexpected response type"),
        }
    }
}
/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
fn manual_verification(geetest: &Geetest) {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Error opening browser: {}", e);
    }
    
}


/******账号密码登录******/


/// 登录盐
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginKey {
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
        loginkey: LoginKey,
        username: String,
        password: String,
        captcha_data: CaptchaData,
        validate: String,
        seccode: String,
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
impl Query for WebLoginQuery{}


/// 登录响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginResponse {
    code: i32,
}
impl Display for WebLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            0 => write!(f, "成功"),
            -105 => write!(f, "验证码错误"),
            -400 => write!(f, "请求错误"),
            -629 => write!(f, "账号或密码错误"),
            -653 => write!(f, "用户名或密码不能为空"),
            -662 => write!(f, "提交超时,请重新提交"),
            -2001 => write!(f, "缺少必要的的参数"),
            -2100 => write!(f, "需验证手机号或邮箱"),
            2400 => write!(f, "登录秘钥错误"),
            2406 => write!(f, "验证极验服务出错"),
            86000 => write!(f, "RSA解密失败"),
            _ => write!(f, "未知错误"),
        }
    }
}

impl Session {
    /// 获取登录秘钥
    pub async fn get_login_key(&self) -> LoginKey {
        let url = "https://passport.bilibili.com/x/passport-login/web/key";
        let response = self
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<ResponseData>()
            .await
            .unwrap()
            .take()
            .unwrap();
        match response {
            Data::LoginKey(login_key) => login_key,
            _ => panic!("Unexpected response type"),
        }
    }
    /// web端登录
    pub async fn web_login(&self, query: String) -> WebLoginResponse {
        let url = "https://passport.bilibili.com/x/passport-login/web/login";
        let url = format!("{}?{}", url, query);
        let response = self
            .post(url)
            .send()
            .await
            .unwrap()
            .json::<WebLoginResponse>()
            .await
            .unwrap();
        response
    }
}

#[cfg(test)]
#[cfg(feature = "manual")]
mod tests {
    use std::io::BufRead;
    use tokio::fs::File;
    use super::*;
    #[tokio::test]
    /// 这个要单独测试，因为需要手动输入验证码
    async fn test_web_login() {
        let session = Session::new();
        let captcha_data = session.captcha().await;
        manual_verification(&captcha_data.geetest);

        let stdin = std::io::stdin();
        let mut validate = String::new();
        stdin.read_line(&mut validate).unwrap();

        let mut seccode = String::new();
        stdin.lock().read_line(&mut seccode).unwrap();


        let mut buf = String::new();
        File::open("test/user.txt")
            .await
            .unwrap()
            .read_to_string(&mut buf)
            .await
            .unwrap();
        let buf: Vec<&str> = buf.split("\n").collect();
        let username = buf[0].to_owned();
        let password = buf[1].to_owned();
        let loginkey = session.get_login_key().await;
        let query = WebLoginQuery::new(
            loginkey,
            username,
            password,
            captcha_data,
            validate,
            seccode,
        )
        .to_query()
        .unwrap();
        let response = session.web_login(query).await;
        assert_eq!(0,response.code)

    }
}
