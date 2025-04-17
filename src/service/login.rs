//! 登陆
//!
//! Example 密码登录
//! ```no_run
//! let (username, password) = ("username", "password");
//! let session = Session::new().unwrap();
//!
//! let captcha = get_captcha(&session).await.unwrap();
//!
//! // 需要启用 manual
//! // 这里会使用默认浏览器跳转到一个过captcha的页面，需要手动验证
//! manual_verification(&captcha.geetest).unwrap();
//!
//! // 将得到的结果 verify 输入到控制台
//! let mut buf = Vec::new();
//! tokio::io::stdin().read_buf(&mut buf).await.unwrap();
//! let validate= String::from_utf8(buf).unwrap().trim();
//!
//! let key = get_login_key(&session).await.unwrap();
//! let password = key.decode_password(password).unwrap();
//! let query = LoginQuery::new(username.to_string(), password.to_string(), captcha, validate.to_string(), None, None);
//! let response = session.login_by_password(query).await.unwrap();
//! println!("登录状态: {}", response.message);
//! // 保存 cookies 到文件
//! session.save_cookies().unwrap();
//! ```

use super::{bili_get, session::Session};
use crate::{
    error::Error,
    model::{
        login::{
            captcha::Captcha,
            password::{LoginKey, LoginState},
        },
        response::BiliResponse,
    },
    query::login::{
        captcha::CAPTCHA_URL,
        password::{LoginQuery, LOGIN_KEY_URL, LOGIN_URL},
    },
};
use bili_core::Query;

/// 获取验证码
pub async fn get_captcha(session: &Session) -> Result<Captcha, Error> {
    bili_get(session, CAPTCHA_URL).await
}

/// 获取登录秘钥
pub async fn get_login_key(session: &Session) -> Result<LoginKey, Error> {
    bili_get(session, LOGIN_KEY_URL).await
}

/// 登录
pub async fn login_by_password(session: &Session, query: LoginQuery) -> Result<LoginState, Error> {
    let url = format!("{}?{}", LOGIN_URL, query.to_query()?);

    session
        .post(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

/// 跳转人工认证页面
/// 外源，可能会失效
#[cfg(feature = "manual")]
use crate::model::login::captcha::Geetest;
#[cfg(feature = "manual")]
pub fn manual_verification(geetest: &Geetest) -> Result<(), Error> {
    let url = "https://kuresaru.github.io/geetest-validator/";
    let url = format!("{}?gt={}&challenge={}", url, geetest.gt, geetest.challenge);
    Ok(webbrowser::open(&url)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_captcha() {
        let session = Session::new().unwrap();
        let captcha = get_captcha(&session).await.unwrap();

        assert!(!captcha.token.is_empty())
    }

    #[tokio::test]
    async fn test_get_login_key() {
        let session = Session::new().unwrap();
        let key = get_login_key(&session).await.unwrap();

        assert!(!key.salt.is_empty())
    }

    #[tokio::test]
    async fn test_login_by_password() {
        let session = Session::new().unwrap();
        let captcha = get_captcha(&session).await.unwrap();
        let query = LoginQuery::new(
            "testuser".to_string(),
            "testpassword".to_string(),
            captcha,
            "validate".to_string(),
            None,
            None,
        );
        let err = login_by_password(&session, query).await;

        assert!(err.is_err());
    }
}
