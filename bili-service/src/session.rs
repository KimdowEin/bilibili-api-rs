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
//! ```ignore
//! let session = Session::new_with_path("./cookies.json").unwrap();
//! ```
//!

use std::sync::Arc;

use arc_swap::ArcSwap;
use derive_more::Deref;
use reqwest::Client;

use crate::{error::Error, state::SessionState};

// pub const COOKIES_URL: &str = "https://api.bilibili.com";

// fn headers() -> header::HeaderMap {
//     let mut headers = header::HeaderMap::new();
//     headers.insert(
//         "referer",
//         header::HeaderValue::from_static("https://www.bilibili.com"),
//     );

//     headers.insert(
//         "User-Agent",
//         header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
//     );
//     headers
// }

#[derive(Debug, Clone, Deref)]
pub struct Session {
    state: Arc<SessionState>,
    #[deref]
    pub client: Client,
    /// sign
    mixin_key: Arc<ArcSwap<String>>,
    /// csrf
    bili_jct: Arc<ArcSwap<String>>,
}

impl Session {
    pub fn new(client: Client, state: Arc<SessionState>) -> Self {
        let mixin_key = Arc::new(ArcSwap::from_pointee(String::new()));
        let bili_jct = Arc::new(ArcSwap::from_pointee(String::new()));

        Self {
            client,
            state,
            mixin_key,
            bili_jct,
        }
    }

    pub async fn save_cookies(&self) -> Result<(), Error> {
        self.state.save_cookies().await
    }

    pub fn get_cookie(&self, domain: &str, key: &str) -> Option<String> {
        self.state.get_cookie(domain, key)
    }

    pub fn set_ticket(&self, ticket: &str) -> Result<(), Error> {
        let cookie_str = format!("bili_ticket={}", ticket);
        self.state
            .set_cookie(&cookie_str, "https://.bilibili.com")?;
        self.state
            .set_cookie(&cookie_str, "https://www.bilibili.com")?;
        self.state
            .set_cookie(&cookie_str, "https://api.bilibili.com")?;
        self.state
            .set_cookie(&cookie_str, "https://live.bilibili.com")?;
        Ok(())
    }

    /// 访问csrf(bili_jct)
    pub fn bili_jct(&self) -> Arc<String> {
        self.bili_jct.load_full()
    }

    /// 设置bili_jct
    pub fn set_bili_jct(&self, bili_jct: &str) {
        self.bili_jct.store(Arc::new(bili_jct.to_string()));
    }

    /// 访问 wbi key
    pub fn mixin_key(&self) -> Arc<String> {
        self.mixin_key.load_full()
    }

    /// 设置 wbi 签名
    pub fn set_mixin_key(&self, mixin_key: &str) {
        self.mixin_key.store(Arc::new(mixin_key.to_string()));
    }

    pub fn refresh_csrf(&self) -> Option<String> {
        self.get_cookie(".bilibili.com", "bili_jct")
            .or(self.get_cookie("www.bilibili.com", "bili_jct"))
            .or(self.get_cookie("api.bilibili.com", "bili_jct"))
            .or(self.get_cookie("live.bilibili.com", "bili_jct"))
            .inspect(|bili_jct| self.set_bili_jct(bili_jct))
    }
}

#[cfg(test)]
mod tests {
    use reqwest::ClientBuilder;

    use super::*;

    #[test]
    pub fn test_refresh_csrf() {
        let state = SessionState::from_path("../cookies_v2.json")
            .map(Arc::new)
            .unwrap();
        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .build()
            .unwrap();
        let session = Session::new(client, state);
        session.refresh_csrf().unwrap();
    }
}
