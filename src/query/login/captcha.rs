//! 人机验证
//! 人机验证流程
//! 1.请求验证码参数，得到登录密钥key与极验idgt和极验KEYchallenge
//! 2.进行滑动or点击验证(feature=manual 提供了一个网站用来手动验证)
//! 3.返回验证结果validate与seccode，进行短信或密码登录
//! '''
//! 
//! '''
use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 申请captcha验证码
pub const CAPTCHA_URL: &str =
    "https://passport.bilibili.com/x/passport-login/captcha?source=main_web";

/// 申请captcha验证码
#[derive(Debug,Default, Serialize, Deserialize)]
pub struct CaptchaQuery {}
impl Query for CaptchaQuery {
}


