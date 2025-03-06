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
    traits::Query,
};

use super::session::Session;

pub async fn captcha(session: &Session) -> Result<Captcha, Error> {
    session
        .get(CAPTCHA_URL)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

/// 获取登录秘钥
pub async fn get_login_key(session: &Session) -> Result<LoginKey, Error> {
    session
        .get(LOGIN_KEY_URL)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

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
    async fn test_captcha() {
        let session = Session::new().unwrap();
        let captcha = captcha(&session).await.unwrap();

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
        let captcha = captcha(&session).await.unwrap();
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
