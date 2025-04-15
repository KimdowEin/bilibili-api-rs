//! 人机验证请求
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

use crate::Query;
use serde::{Deserialize, Serialize};

/// 申请captcha验证码
pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";

/// 申请captcha验证码
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct CaptchaQuery;

impl CaptchaQuery {
    pub fn new() -> Self {
        Self
    }
}
