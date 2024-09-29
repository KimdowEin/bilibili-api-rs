#![allow(dead_code)]

use crate::{
    login::{
        action::{CaptchaData, LoginKey},
        info::NavData,
    },
    sign::wbi::WbiSign,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};

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

pub trait Query: Serialize + Sized {
    /// 生成原始query
    fn to_query(&self) -> Result<String, serde_qs::Error> {
        serde_qs::to_string(self)
    }

    /// 生成需要签名的query
    fn sign(&self, mixin_key: &str) -> String
    where
        Self: WbiSign,
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let wts = format!("wts={}", timestamp);
        let ori_query = self.to_query().unwrap();
        let mut querys = ori_query.split("&").collect::<Vec<&str>>();
        querys.push(&wts);
        querys.sort();
        let mut hash_query = querys.join("&");
        hash_query.push_str(mixin_key);

        let w_rid = format!("w_rid={:?}", md5::compute(&hash_query));
        let query = format!("{}&{}&{}", ori_query, w_rid, wts);
        query
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::sign::wbi::WbiSign;

    use super::*;

    #[derive(Serialize)]
    struct TestQuery {
        bar: i64,
        foo: i64,
        zab: i64,
    }
    impl WbiSign for TestQuery {}
    impl Query for TestQuery {
        fn sign(&self, mixin_key: &str) -> String
        where
            Self: WbiSign,
        {
            let timestamp =1702204169;
            let wts = format!("wts={}", timestamp);
            let ori_query = self.to_query().unwrap();
            let mut querys = ori_query.split("&").collect::<Vec<&str>>();
            querys.push(&wts);
            querys.sort();
            let mut hash_query = querys.join("&");
            hash_query.push_str(mixin_key);

            let w_rid = format!("w_rid={:?}", md5::compute(&hash_query));
            assert_eq!(w_rid, "w_rid=8f6f2b5b3d485fe1886cec6a0be8c5d4");

            let query = format!("{}&{}&{}", ori_query, w_rid, wts);
            query
        }
    }

    #[test]
    fn w_rid_test() {
        let query = TestQuery {
            bar: 514,
            foo: 114,
            zab: 1919810,
        };
        let mixin_key = "ea1db124af3c7062474693fa704f4ff8";
        query.sign(mixin_key);
    }
}
