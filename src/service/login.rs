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

impl Session {
    pub async fn captcha(&self) -> Result<Captcha, Error> {
        self.get(CAPTCHA_URL)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    pub async fn login_by_password(&self, query: LoginQuery) -> Result<LoginState, Error> {
        let url = format!("{}?{}", LOGIN_URL, query.to_query()?);
        self.post(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }
    /// 获取登录秘钥
    pub async fn get_login_key(&self) -> Result<LoginKey, Error> {
        self.get(LOGIN_KEY_URL)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }
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
