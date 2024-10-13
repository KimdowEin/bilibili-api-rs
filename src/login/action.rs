#![allow(dead_code)]
////////////////
/// 登陆操作 ///
////////////////
pub mod captcha;
pub mod password;

use std::fmt::Display;

use captcha::CaptchaData;
use password::{LoginKeyData, WebLoginData};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginResponse {
    pub code: WebLoginResponseCode,
    pub message: String,
    pub data: Option<LoginData>,
}

impl Response for WebLoginResponse {
    type Data = LoginData;
    fn is_success(&self) -> bool {
        self.code == WebLoginResponseCode::Success
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn data(self) -> Self::Data {
        self.data.unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
