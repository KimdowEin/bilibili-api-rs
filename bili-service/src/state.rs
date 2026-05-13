use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
    sync::Arc,
};

use cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use serde::Deserialize;
use url::Url;

use crate::error::Error;

#[derive(Debug)]
pub struct SessionState {
    pub store: Arc<CookieStoreMutex>,
}
impl Default for SessionState {
    fn default() -> Self {
        Self {
            store: Arc::new(CookieStoreMutex::default()),
        }
    }
}

impl SessionState {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let store = load_cookies_v2(&path)
            .or_else(|_| load_cookies_v1(&path))
            .map(reqwest_cookie_store::CookieStoreMutex::new)
            .map(Arc::new)?;

        Ok(Self { store })
    }

    pub fn set_cookie(&self, cookie_str: &str, url: &str) -> Result<(), Error> {
        let url = Url::parse(url)?;
        let cookie = cookie_store::Cookie::parse(cookie_str.to_string(), &url)?;

        self.store.lock().expect("").insert(cookie, &url)?;
        Ok(())
    }

    pub fn get_cookie(&self, domain: &str, key: &str) -> Option<String> {
        self.store
            .lock()
            .expect("")
            .get_any(domain, "/", key)
            .map(|cookie| cookie.value().to_string())
    }

    pub async fn save_cookies(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = File::create(&path).map(BufWriter::new)?;
        let store = self.store.lock().expect("");

        cookie_store::serde::json::save_incl_expired_and_nonpersistent(&store, &mut writer)?;

        writer.flush()?;
        Ok(())
    }
}

pub fn load_cookies_v2(path: impl AsRef<Path>) -> Result<CookieStore, Error> {
    let reader = File::open(&path).map(BufReader::new)?;
    let store = cookie_store::serde::json::load(reader)?;

    Ok(store)
}

#[derive(Debug, Deserialize)]
struct V1CookieEntry {
    url: String,
    cookies: String,
}
pub fn load_cookies_v1(path: impl AsRef<Path>) -> Result<CookieStore, Error> {
    let reader = File::open(path).map(BufReader::new)?;

    let mut store = cookie_store::CookieStore::new();
    for entry in serde_json::from_reader::<_, Vec<V1CookieEntry>>(reader)? {
        let url = Url::parse(&entry.url)?;
        entry
            .cookies
            .split(';')
            .map(|cookie_str| cookie_str.trim())
            .filter(|cookie_str| !cookie_str.is_empty())
            .map(|cookie_str| cookie_store::Cookie::parse(cookie_str, &url))
            .try_for_each(|cookie| -> Result<_, Error> {
                let cookie = cookie?;
                store.insert(cookie.into_owned(), &url)?;
                Ok(())
            })?;
    }

    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_load_cookies_v1() {
        load_cookies_v1("../cookies.json").unwrap();
    }

    #[test]
    #[ignore]
    fn test_cookies_v1_to_v2() {
        let store = load_cookies_v1("../cookies.json").unwrap();
        let mut writer = File::create("../cookies_v2.json")
            .map(BufWriter::new)
            .unwrap();
        cookie_store::serde::json::save_incl_expired_and_nonpersistent(&store, &mut writer)
            .unwrap();

        writer.flush().unwrap();
    }
    #[test]
    #[ignore]
    fn test_load_cookies_v2() {
        load_cookies_v2("../cookies_v2.json").unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_session_state() {
        let state = SessionState::from_path("../cookies.json").unwrap();
        state.save_cookies("../cookies_v2.json").await.unwrap();

        SessionState::from_path("../cookies_v2.json")
            .unwrap()
            .save_cookies("../cookies_v2.json")
            .await
            .unwrap();
    }
}
