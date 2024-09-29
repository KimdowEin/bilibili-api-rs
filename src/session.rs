#![allow(dead_code)]

use crate::login::{action::{CaptchaData, LoginKey}, info::NavData};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// 会话信息
/// mixin_key 需要登录后才有
/// 每日刷新一次
/// 每次web端请求要使用w_rid函数生成query
pub struct Session {
    client: Client,
    pub mixin_key: String,
}
impl Session {
    pub fn new() -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        let mixin_key = String::new();
        Self { client, mixin_key }
    }
}

impl Deref for Session {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    code: i64,
    message: String,
    data: Data,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Data {
    NavData(NavData),
    CaptchaData(CaptchaData),
    LoginKey(LoginKey),
    None,
}
impl ResponseData {
    pub fn take(self) -> Option<Data> {
        match self.data {
            Data::None => None,
            _ => Some(self.data),
        }
    }
}
