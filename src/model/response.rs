use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct BiliResponse<T> {
    pub code: BiliResponseCode,
    pub message: String,
    pub data: Option<T>,
}
impl<T> BiliResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == BiliResponseCode::Success
    }

    pub fn data(self) -> Result<T, Error> {
        if self.is_success() {
            self.data.ok_or(Error::from("No data in response"))
        } else {
            Err(Error::QueryError(self.message))
        }
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq,Eq)]
#[repr(i32)]
pub enum BiliResponseCode {
    Success = 0,

    APIKeyError = -3,

    NotLogin = -101,
    AccountSuspended = -102,
    CoinShortage = -104,
    CaptchaError = -105,
    CsrfError = -111,

    DangerError = -352,

    RequestError = -400,
    AccountException = -403,
    VideoNotFound = -404,

    PasswordError = -629,
    AccountEmpty = -653,
    PostTimeout = -662,

    MissingParams = -2001,
    NeedVerification = -2100,

    LiveRoomNotFound = 1,

    LoginKeyError = 2400,
    GeetestError = 2406,

    VideoNotFound2 = 10003,

    CoinToSelf = 34002,
    CoinNumIllegal = 34003,
    CoinDurationError = 34004,
    CoinTooMuch = 34005,

    UserSingAllergy = 40015,
    UserSingHasEmoji = 40021,
    UserSingTooLong = 40022,

    VideoInvisible = 62002,
    LikeCancelFailed = 65004,
    LikeAgain = 65006,

    RsaDecryptFail = 86000,

    LiveRoomInfoNotFound = 19002003,
    ArgsError = 2001000,

    #[serde(other)]
    Unknown,
}

impl Display for BiliResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiliResponseCode::Success => write!(f, "成功"),

            BiliResponseCode::AccountEmpty => write!(f, "账号为空"),
            BiliResponseCode::AccountException => write!(f, "账号异常"),
            BiliResponseCode::AccountSuspended => write!(f, "账号被封禁"),
            BiliResponseCode::APIKeyError => write!(f, "API校验密匙错误"),
            BiliResponseCode::ArgsError => write!(f, "参数错误"),

            BiliResponseCode::CaptchaError => write!(f, "验证码错误"),
            BiliResponseCode::CoinDurationError => write!(f, "投币时间间隔太短"),
            BiliResponseCode::CsrfError => write!(f, "csrf校验失败"),
            BiliResponseCode::CoinShortage => write!(f, "硬币不足"),
            BiliResponseCode::CoinNumIllegal => write!(f, "投币数量不合法"),
            BiliResponseCode::CoinToSelf => write!(f, "不能给自己投币"),
            BiliResponseCode::CoinTooMuch => write!(f, "投币数量超过限制"),

            BiliResponseCode::DangerError => write!(f, "风控错误"),
            BiliResponseCode::GeetestError => write!(f, "极验服务出错"),
            
            BiliResponseCode::LikeCancelFailed => write!(f, "取消点赞失败"),
            BiliResponseCode::LikeAgain => write!(f, "已经点赞过了"),
            BiliResponseCode::LiveRoomInfoNotFound => write!(f, "房间信息不存在"),
            BiliResponseCode::LiveRoomNotFound => write!(f, "直播间不存在"),
            BiliResponseCode::LoginKeyError => write!(f, "登录密匙错误"),

            BiliResponseCode::MissingParams => write!(f, "缺少参数"),

            BiliResponseCode::NeedVerification => write!(f, "需要验证"),
            BiliResponseCode::NotLogin => write!(f, "账号未登录"),

            BiliResponseCode::PasswordError => write!(f, "密码错误"),
            BiliResponseCode::PostTimeout => write!(f, "提交超时"),

            BiliResponseCode::RequestError => write!(f, "请求错误"),
            BiliResponseCode::RsaDecryptFail => write!(f, "RSA解密失败"),

            BiliResponseCode::UserSingAllergy => write!(f, "用户签名包含敏感词"),
            BiliResponseCode::UserSingHasEmoji => write!(f, "用户签名包含emoji"),
            BiliResponseCode::UserSingTooLong => write!(f, "用户签名过长"),

            BiliResponseCode::VideoInvisible => write!(f, "视频不可见"),
            BiliResponseCode::VideoNotFound => write!(f, "视频不存在"),
            BiliResponseCode::VideoNotFound2 => write!(f, "视频不存在"),

            BiliResponseCode::Unknown => write!(f, "其他错误"),
        }
    }
}
