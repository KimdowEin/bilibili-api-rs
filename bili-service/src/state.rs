use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use serde::Deserialize;
use url::Url;

use crate::error::Error;

#[derive(Debug)]
pub struct SessionState {
    path: Option<PathBuf>,
    pub store: Arc<CookieStoreMutex>,
}
impl Default for SessionState {
    fn default() -> Self {
        Self {
            path: Some(PathBuf::from("cookies.json")),
            store: Arc::new(CookieStoreMutex::default()),
        }
    }
}

impl SessionState {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let store = load_cookies_v2(&path)
            .or_else(|_| load_cookies_v1(&path))
            .ok()
            .or_else(|| Some(reqwest_cookie_store::CookieStore::new()))
            .map(reqwest_cookie_store::CookieStoreMutex::new)
            .map(Arc::new)
            .expect("there will always be success");

        let path = Some(path.as_ref().to_path_buf());

        Ok(Self { store, path })
    }

    pub fn set_path(&mut self, path: impl AsRef<Path>) {
        self.path.replace(path.as_ref().to_path_buf());
    }
    pub fn set_cookie(&self, cookie_str: &str, url: &str) -> Result<(), Error> {
        let url = Url::parse(url)?;
        let cookie = cookie_store::Cookie::parse(cookie_str.to_string(), &url)?;

        self.store.lock().unwrap().insert(cookie, &url)?;
        Ok(())
    }

    pub fn get_cookie(&self, domain: &str, key: &str) -> Option<String> {
        self.store
            .lock()
            .unwrap()
            .get_any(domain, "/", key)
            .map(|cookie| cookie.value().to_string())
    }

    pub async fn save_cookies(&self) -> Result<(), Error> {
        let path = self.path.clone().unwrap_or("./cookies_v2.json".into());
        let mut writer = File::create(&path).map(BufWriter::new)?;
        let store = self.store.lock().unwrap();

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
        let mut state = SessionState::from_path("../cookies.json").unwrap();
        state.set_path("../cookies_v2.json");
        state.save_cookies().await.unwrap();

        SessionState::from_path("../cookies_v2.json")
            .unwrap()
            .save_cookies()
            .await
            .unwrap();
    }
}
