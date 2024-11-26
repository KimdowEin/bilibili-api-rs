//！登陆操作
//! 登陆操作包含 扫码登陆(未实现) 与 验证登录(create password)

#![allow(dead_code)]
pub mod captcha;
pub mod password;

use std::fmt::Display;

use captcha::CaptchaData;
use password::{LoginKeyData, WebLoginData};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::Response;


/// Web端登陆响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginResponse {
    code: WebLoginResponseCode,
    message: String,
    // ttl:u8,
    data: Option<LoginData>,
}

impl Response for WebLoginResponse {
    type Data = Option<LoginData>;
    fn is_success(&self) -> bool {
        self.code == WebLoginResponseCode::Success
    }
    fn message(self) -> String {
        self.message
    }
    fn data(self) ->Self::Data {
        self.data
    }
}
impl Display for WebLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginData {
    CaptchaData(CaptchaData),
    LoginKeyData(LoginKeyData),
    WebLoginData(WebLoginData),
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum WebLoginResponseCode {
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
impl Display for WebLoginResponseCode {
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

// #[cfg(test)]
// mod tests {
//     use crate::common::Session;

//     #[tokio::test]
//     async fn test_login_action_captcha() {
//         Session::new().captcha().await.unwrap();
//     }

//     #[tokio::test]
//     async fn test_login_action_get_login_key() {
//         Session::new().get_login_key().await.unwrap();
//     }

//     #[tokio::test]
//     #[cfg(feature = "manual")]
//     async fn test_login_action_web_login() {
//         use std::{
//             fs::File,
//             io::{BufRead, Read},
//             thread,
//             time::Duration,
//         };

//         use crate::login::action::{captcha::manual_verification, password::WebLoginQuery};

//         let mut session = Session::new();
//         let captcha_data = session.captcha().await.unwrap();
//         manual_verification(&captcha_data.geetest).unwrap();
//         thread::sleep(Duration::from_secs(10));
//         session.heartbeat().await.unwrap();

//         let stdin = std::io::stdin();
//         let mut validate = String::new();
//         stdin.lock().read_line(&mut validate).unwrap();
//         let validate = validate.trim().to_owned();

//         let mut buf = String::new();
//         File::open("./tests/res/user.txt")
//             .unwrap()
//             .read_to_string(&mut buf)
//             .unwrap();

//         let buf = buf.lines().collect::<Vec<&str>>();
//         let username = buf[0].to_owned();
//         let password = buf[1].to_owned();

//         let loginkey = session.get_login_key().await.unwrap();
//         let query = WebLoginQuery::new(loginkey, username, password, captcha_data, validate);

//         let web_login_data = session.web_login(query).await.unwrap();
//         println!("web_login_data: {}", web_login_data.message);

//         // 存key
//         session.get_mixin_key().await.unwrap();
//         let key = session.key();
//         let _ = std::fs::write("test/key.txt", key);
//     }
// }
