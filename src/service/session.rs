//! 会话管理
//! 
//! 用于与服务器进行交互
//! 
//! 已经封装好了cookies的处理逻辑,
//! 不建议再造轮子
//! 
//! 从浏览器复制cookie
//! ```json
//! ./cookies.json
//! [
//! {
//!   "url":"https://api.bilibili.com",
//!   "cookies":"a=abcdefg; b=hijklmn"
//! }
//! ]
//! ```
//! 
//! 创建会话
//! ```no_run
//! let session = Session::new_with_path("./cookies.json").unwrap();
//! ```
//! 
//! 或者new_with_client,可以更精细的控制

use std::{
    fs::File,
    io::{BufReader, Write},
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};
use bili_core::{Csrf, Data, Query, Sign};
use reqwest::{
    cookie::{CookieStore, Jar},
    header, Client, Url,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::{error::Error, model::response::BiliResponse};

pub const COOKIES_URL: &str = "https://api.bilibili.com";

pub fn headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "referer",
        header::HeaderValue::from_static("https://live.bilibili.com"),
    );

    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
    );
    headers
}

#[derive(Debug, Clone)]
pub struct Session {
    pub state: Arc<SessionState>,
    pub client: Client,
    pub mixin_key: Arc<RwLock<String>>,
    pub bili_jct: Arc<RwLock<String>>,
}

impl Deref for Session {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Session {
    pub fn new() -> Result<Self, Error> {
        let headers = headers();

        let state = Arc::new(SessionState::default());
        let mixin_key = Arc::new(RwLock::new(String::new()));
        let bili_jct = Arc::new(RwLock::new(String::new()));
        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(state.jar.clone())
            .build()?;

        Ok(Self {
            client,
            state,
            mixin_key,
            bili_jct,
        })
    }

    pub fn new_with_client(client: Client, state: Arc<SessionState>) -> Self {
        let mixin_key = Arc::new(RwLock::new(String::new()));
        let bili_jct = Arc::new(RwLock::new(String::new()));

        Self {
            client,
            state,
            mixin_key,
            bili_jct,
        }
    }

    /// 从文件中读取cookies,headers用的是本库的实现
    pub fn new_with_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let state = Arc::new(SessionState::from_path(path)?);
        let headers = headers();

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(state.jar.clone())
            .build()?;

        let mixin_key = Arc::new(RwLock::new(String::new()));
        let bili_jct = state.get_cookie(COOKIES_URL, "bili_jct");
        let bili_jct = if let Some(bili_jct) = bili_jct {
            Arc::new(RwLock::new(bili_jct))
        } else {
            Arc::new(RwLock::new(String::new()))
        };

        Ok(Self {
            client,
            state,
            mixin_key,
            bili_jct,
        })
    }

    pub fn save_cookies(&self) -> Result<(), Error> {
        self.state.save_cookies()
    }

    pub fn get_cookie(&self, url: &str, key: &str) -> Option<String> {
        self.state.get_cookie(url, key)
    }

    pub fn set_ticket(&self, ticket: &str) {
        let url = Url::parse(COOKIES_URL).unwrap();
        self.state.jar.add_cookie_str(ticket, &url);
    }
}

#[derive(Debug)]
pub struct SessionState {
    pub cookies_path: Option<PathBuf>,
    pub jar: Arc<Jar>,
}
impl Default for SessionState {
    fn default() -> Self {
        Self {
            cookies_path: Some(PathBuf::from("cookies.json")),
            jar: Arc::new(Jar::default()),
        }
    }
}

impl SessionState {
    pub fn from_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(&path)?;
        let reader = BufReader::new(&mut file);
        let cookies: Vec<CookieItem> = serde_json::from_reader(reader)?;

        let path = Some(path.as_ref().to_path_buf());

        let state = Self {
            jar: Arc::new(Jar::default()),
            cookies_path: path,
        };

        cookies.into_iter().for_each(|cookie| {
            let CookieItem { url, cookies } = cookie;
            let url = Url::parse(&url).unwrap();

            cookies.split(";").map(|x| x.trim()).for_each(|cookie| {
                state.jar.add_cookie_str(cookie, &url);
            });
        });
        Ok(state)
    }

    pub fn get_cookie(&self, url: &str, key: &str) -> Option<String> {
        let url = Url::parse(url).unwrap();
        let cookies = self.jar.cookies(&url);
        let cookies = if let Some(cookies) = cookies {
            cookies
        } else {
            return None;
        };

        let cookies = cookies.to_str().unwrap();
        for c in cookies.split(';') {
            let mut parts = c.splitn(2, '=');
            if let (Some(c_key), Some(value)) = (parts.next(), parts.next()) {
                if c_key.trim() == key.trim() {
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    pub fn save_cookies(&self) -> Result<(), Error> {
        if let Some(path) = &self.cookies_path {
            let mut file = File::create(path)?;
            let mut cookies = Vec::new();
            let to_url = Url::parse(COOKIES_URL).unwrap();
            let cookie = self.jar.cookies(&to_url);
            if let Some(cookie) = cookie {
                let cookie = cookie.to_str().unwrap();

                let cookie_item = CookieItem {
                    url: COOKIES_URL.to_string(),
                    cookies: cookie.to_string(),
                };
                cookies.push(cookie_item);
            }

            let json = serde_json::to_string(&cookies)?;
            file.write_all(json.as_bytes())?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct CookieItem {
    url: String,
    cookies: String,
}

pub async fn bili_get<D>(session: &Session, url: &str) -> Result<D, Error>
where
    D: Data,
{
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn bili_query_get<D, Q>(session: &Session, url: &str, query: Q) -> Result<D, Error>
where
    D: Data,
    Q: Query,
{
    let url = format!("{}?{}", url, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn bili_sign_get<D, Q>(session: &Session, url: &str, query: Q) -> Result<D, Error>
where
    D: Data,
    Q: Query + Sign,
{
    let bili_jct = session.bili_jct().await;
    let url = format!("{}?{}", url, query.sign(&bili_jct)?);

    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}
pub async fn bili_csrf_get<D, Q>(session: &Session, url: &str, query: Q) -> Result<D, Error>
where
    D: Data,
    Q: Query + Csrf,
{
    let bili_jct = session.bili_jct().await;
    let url = format!("{}?{}", url, query.csrf(&bili_jct)?);

    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}
