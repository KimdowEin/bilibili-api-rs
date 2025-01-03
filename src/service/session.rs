use std::{
    fs::File, io::{BufReader, Write}, ops::Deref, path::{Path, PathBuf}, sync::Arc
};

use reqwest::{
    cookie::{CookieStore, Jar},
    header, Client, Url,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::error::Error;

const COOKIES_URLS: &str = "https://api.bilibili.com";

pub fn headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "referer",
        header::HeaderValue::from_static("https://www.bilibili.com"),
    );

    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
    );
    headers
}


pub struct Session {
    pub state: Arc<SessionState>,
    pub client: Client,
    pub mixin_key: RwLock<String>,
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
        let mixin_key = RwLock::new(String::new());
        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(state.jar.clone())
            .build()?;

        Ok(Self {
            client,
            state,
            mixin_key,
        })
    }

    pub fn new_with_client(client: Client, state: Arc<SessionState>) -> Self {
        let mixin_key = RwLock::new(String::new());

        Self {
            client,
            state,
            mixin_key,
        }
    }

    pub fn new_with_path<P>(path: P) -> Result<Self, Error> 
    where P: AsRef<Path>,{
        let mut file = File::open(&path)?;
        let reader = BufReader::new(&mut file);
        let cookies: Vec<CookieItem> = serde_json::from_reader(reader)?;

        let state = Arc::new(SessionState {
            jar: Arc::new(Jar::default()),
            cookies_path: path.as_ref().to_path_buf(),
        });

        cookies.into_iter().for_each(|cookie| {
            let CookieItem { url, cookies } = cookie;
            let url = Url::parse(&url).unwrap();

            cookies
                .split(";")
                .map(|x| x.trim())
                .for_each(|cookie| {
                    state.jar.add_cookie_str(cookie, &url);
                });
        });

        let headers = headers();

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(state.jar.clone())
            .build()?;

        let mixin_key = RwLock::new(String::new());

        Ok(Self {
            client,
            state,
            mixin_key,
        })
    }

    pub fn save_cookies(&self) -> Result<(), Error> {
        let mut file = File::create(&self.state.cookies_path)?;
        let mut cookies = Vec::new();
        let to_url = Url::parse(COOKIES_URLS).unwrap();
        let cookie = self.state.jar.cookies(&to_url);
        if let Some(cookie) = cookie {
            let cookie = cookie.to_str().unwrap();

            let cookie_item = CookieItem {
                url: COOKIES_URLS.to_string(),
                cookies: cookie.to_string(),
            };
            cookies.push(cookie_item);
        }

        let json = serde_json::to_string(&cookies)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SessionState {
    pub cookies_path: PathBuf,
    pub jar: Arc<Jar>,
}
impl Default for SessionState {
    fn default() -> Self {
        Self {
            cookies_path: PathBuf::from("cookies.json"),
            jar: Arc::new(Jar::default()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct CookieItem {
    url: String,
    cookies: String,
}
