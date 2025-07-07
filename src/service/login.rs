//! 登陆
//!
//! Example 密码登录
//! ```ignore
//! let (username, password) = ("username", "password");
//! let session = Session::new().unwrap();
//!
//! let query = CaptchaQuery::new();
//! let captcha = CaptchaRequest::send_request(&session, query).await.unwrap();
//!
//! // 这里会使用默认浏览器跳转到一个过captcha的页面，需要手动验证
//! manual_verification(&captcha.geetest).unwrap();
//!
//! // 将得到的结果 verify 输入到控制台
//! let mut buf = String::new();
//! std::io::stdin().read_line(&mut buf).unwrap();
//! let validate = buf.trim();
//!
//! let query = LoginKeyQuery::new();
//! let key = LoginKeyRequest::send_request(&session, query)
//!     .await
//!     .unwrap();
//! let password = key.decode_password(password).unwrap();
//!
//! let query = PasswordLoginQuery::new(
//!     username.to_string(),
//!     password,
//!     captcha,
//!     validate.to_string(),
//!     None,
//!     None,
//! );
//! let response = PasswordLoginRequest::send_request(&session, query)
//!     .await
//!     .unwrap();
//!
//! println!("登录状态: {}", response.message);
//!
//! session.save_cookies().unwrap();
//! ```

use_bili_request!();
use crate::{
    model::login::{
        captcha::Captcha,
        password::{LoginKey, PasswordLogin},
    },
    query::login::{
        captcha::{CaptchaQuery, CAPTCHA_URL},
        password::{LoginKeyQuery, PasswordLoginQuery, LOGIN_KEY_URL, PASSWORD_LOGIN_URL},
    },
    use_bili_request,
};

define_bili_request!(Captcha, CAPTCHA_URL, Get);
define_bili_request!(LoginKey, LOGIN_KEY_URL, Get);
define_bili_request!(PasswordLogin, PASSWORD_LOGIN_URL, Post);

/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
use crate::{error::Error, model::login::captcha::Geetest};
#[cfg(feature = "manual")]
pub fn manual_verification(geetest: &Geetest) -> Result<(), Error> {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    Ok(webbrowser::open(&url)?)
}

#[cfg(test)]
mod tests {
    use crate::{query::login::password::LoginKeyQuery, service::Session};

    use super::*;

    #[tokio::test]
    async fn test_get_captcha() {
        let session = Session::new().unwrap();
        let captcha = CaptchaRequest::send_request(&session, CaptchaQuery::new())
            .await
            .unwrap();

        assert!(!captcha.token.is_empty())
    }

    #[tokio::test]
    async fn test_get_login_key() {
        let session = Session::new().unwrap();
        let key = LoginKeyRequest::send_request(&session, LoginKeyQuery::new())
            .await
            .unwrap();

        assert!(!key.salt.is_empty())
    }

    #[tokio::test]
    async fn test_login_by_password() {
        let session = Session::new().unwrap();
        let captcha = CaptchaRequest::send_request(&session, CaptchaQuery::new())
            .await
            .unwrap();
        let query = PasswordLoginQuery::new(
            "testuser".to_string(),
            "testpassword".to_string(),
            captcha,
            "validate".to_string(),
            None,
            None,
        );
        let err = PasswordLoginRequest::send_request(&session, query).await;

        assert!(err.is_err());
    }
}
