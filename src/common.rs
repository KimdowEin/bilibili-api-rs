#![allow(dead_code)]

use crate::{
    login::{
        action::{CaptchaData, LoginKeyData},
        info::NavData,
    },
    sign::wbi::WbiSign,
    video::{info::video_info::WebVideoInfoData, stream::WebPlayUrlData},
};

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};


#[cfg(feature = "session")]
mod session {
    pub use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client, Error,
    };
    pub use std::ops::Deref;

    /// 会话信息
    /// mixin_key 需要登录后才有
    /// 每日刷新一次
    /// 每次web端请求要使用w_rid函数生成query
    pub struct Session {
        client: Client,
        mixin_key: String,
    }
    impl Session {
        pub fn new() -> Self {
            let mut headers = HeaderMap::new();
            headers.insert(
            "User-Agent", 
            HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0").unwrap()
        );
            headers.insert(
                "referer",
                HeaderValue::from_str("https://www.bilibili.com/").unwrap(),
            );

            let client = Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .pool_idle_timeout(None)
                .build()
                .unwrap();

            let mixin_key = String::new();
            Self { client, mixin_key }
        }
        /// 设置 wbi 签名
        pub fn set_mixin_key(&mut self, mixin_key: String) {
            self.mixin_key = mixin_key;
        }
        /// 获取 wbi key
        pub fn key(&self) -> String {
            self.mixin_key.clone()
        }
        /// 心跳,保持连接
        pub async fn heartbeat(&self) -> Result<(), Error> {
            let url = "https://api.bilibili.com/x/web-interface/nav";
            self.get(url).send().await?;
            Ok(())
        }
    }
    impl Deref for Session {
        type Target = Client;
        fn deref(&self) -> &Self::Target {
            &self.client
        }
    }
}

#[cfg(feature="session")]
pub use session::*;


#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    code: i64,
    message: String,
    ttl: u8,
    data: Data,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Data {
    NavData(NavData),
    CaptchaData(CaptchaData),
    LoginKey(LoginKeyData),
    WebPlayUrlData(WebPlayUrlData),
    WebVideoInfoData(WebVideoInfoData),
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
