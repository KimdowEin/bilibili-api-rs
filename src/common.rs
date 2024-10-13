#![allow(dead_code)]

use crate::error::ResponseError;

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub trait WbiSign {}

pub trait Response: Sized {
    type Data;

    fn is_success(&self) -> bool;
    fn message(&self) -> String;
    fn data(self) -> Self::Data;

    // 根据code判断是否成功
    fn result(self) -> Result<Self::Data, ResponseError> {
        if self.is_success() {
            // 假设code为0表示成功
            Ok(self.data())
        } else {
            Err(self.message().into())
        }
    }
}

#[cfg(feature = "session")]
mod session {
    use std::ops::Deref;

    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client, Error,
    };

    /// 会话信息
    /// mixin_key 需要登录后才有
    /// 每日刷新一次
    /// 每次web端请求要使用w_rid函数生成query
    #[derive(Debug, Clone)]
    pub struct Session {
        client: Client,
        pub mixin_key: String,
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

#[cfg(feature = "session")]
pub use session::Session;
