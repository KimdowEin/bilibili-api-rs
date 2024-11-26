#![allow(dead_code)]

use crate::error::Error;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait WbiSign {}

pub trait Query: Serialize + Sized {
    /// 生成原始query
    fn to_query(&self) -> Result<String, Error> {
        Ok(serde_qs::to_string(self)?)
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

    // / 完整的请求url
    // fn url(&self, url: &str) -> String;
}


pub trait Response: DeserializeOwned + Sized {
    type Data;

    fn is_success(&self) -> bool;
    fn message(self) -> String;
    fn data(self) -> Self::Data;

    // 根据code判断是否成功
    fn result(self) -> Result<Self::Data, Error> {
        if self.is_success() {
            // 假设code为0表示成功
            Ok(self.data())
        } else {
            Err(Error::ResponseError(self.message()))
        }
    }
}

#[cfg(feature = "session")]
mod session {
    use std::{
        fs::{self, File},
        io::{BufReader, Read},
        ops::Deref,
        path::PathBuf,
        sync::Arc,
    };

    use cookie_store::CookieStore;
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use reqwest_cookie_store::CookieStoreMutex;

    use crate::error::Error;

    #[derive(Debug, Clone, Default)]
    pub struct State {
        cookie_store_path: Option<PathBuf>,
        cookie_store: Arc<CookieStoreMutex>,
    }

    impl State {
        pub fn new(cookie_store_path: Option<PathBuf>) -> Self {
            let mut cookie_store = CookieStore::default();

            if let Some(path) = &cookie_store_path {
                if let Ok(file) = File::open(path) {
                    if let Ok(cookiestore) = CookieStore::load_json(BufReader::new(file)) {
                        cookie_store = cookiestore;
                    }
                }
            }

            let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));
            Self {
                cookie_store_path,
                cookie_store,
            }
        }
    }
    /// 会话信息
    /// mixin_key 需要登录后才有
    /// 每日刷新一次
    /// 每次web端请求要使用w_rid函数生成query
    #[derive(Debug, Clone)]
    pub struct Session {
        state: Arc<State>,
        client: Client,
        pub mixin_key: String,
    }
    impl Session {
        pub fn new() -> Self {
            let state = Arc::new(State::default());

            let headers = {
                let mut headers = HeaderMap::new();
                headers.insert(
                "User-Agent", 
                HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0").unwrap()
                );
                headers.insert(
                    "referer",
                    HeaderValue::from_str("https://www.bilibili.com/").unwrap(),
                );
                headers
            };

            let client = {
                let client = Client::builder()
                    .cookie_store(true)
                    .default_headers(headers)
                    .pool_idle_timeout(None)
                    .build()
                    .unwrap();
                client
            };

            let mixin_key = String::new();
            Self {
                state,
                client,
                mixin_key,
            }
        }

        pub fn new_with_cookie<P>(cookie_path: P) -> Self
        where
            P: Into<Option<PathBuf>>,
        {
            let state = Arc::new(State::new(cookie_path.into()));

            let headers = {
                let mut headers = HeaderMap::new();
                headers.insert(
                "User-Agent", 
                HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0").unwrap()
                );
                headers.insert(
                    "referer",
                    HeaderValue::from_str("https://www.bilibili.com/").unwrap(),
                );
                headers
            };

            let client = {
                let client = Client::builder()
                    .cookie_store(true)
                    .default_headers(headers)
                    .pool_idle_timeout(None)
                    .build()
                    .unwrap();
                client
            };

            let mixin_key = String::new();
            Self {
                state,
                client,
                mixin_key,
            }
        }

        pub fn with_mixin_key(mut self, mixin_key: String) -> Self {
            self.mixin_key = mixin_key;
            self
        }

        pub fn with_mixin_key_path<P>(mut self, path: P) -> Result<Self, Error>
        where
            P: Into<PathBuf>,
        {
            let mut buf = String::new();
            File::open(path.into())?.read_to_string(&mut buf)?;
            self.mixin_key = buf;
            Ok(self)
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

    impl Drop for State {
        fn drop(&mut self) {
            let path = if let Some(path) = &self.cookie_store_path {
                path
            } else {
                return;
            };

            let file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path);
            let store = self.cookie_store.lock().unwrap();
            file.and_then(|mut file| Ok(store.save_json(&mut file)));
        }
    }
}

#[cfg(feature = "session")]
pub use session::Session;
