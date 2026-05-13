//! 内部测试工具，不对外发布

use std::sync::Arc;

use bili_service::{Session, SessionState};
use reqwest::{
    ClientBuilder,
    header::{self, HeaderMap, HeaderValue},
};

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Linux; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
        ),
    );
    headers.insert(header::ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh-TW;q=0.9,zh;q=0.8,fr;q=0.7,en;q=0.6,ja;q=0.5"),
    );
    headers.insert(header::CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        header::REFERER,
        HeaderValue::from_static("https://www.bilibili.com/"),
    );
    headers
}

fn build_session(state: Arc<SessionState>) -> Session {
    let client = ClientBuilder::new()
        .cookie_provider(state.store.clone())
        .default_headers(default_headers())
        .build()
        .expect("构建 reqwest Client 失败");
    Session::new(client, state)
}

/// 从 cookie 文件路径直接创建带默认 headers 的 Session
pub fn session_from_path(path: impl AsRef<std::path::Path>) -> Session {
    let state = SessionState::from_path(path)
        .map(Arc::new)
        .expect("加载 cookies 失败");
    build_session(state)
}

/// 从已有 state 创建带默认 headers 的 Session
pub fn session_from_state(state: SessionState) -> Session {
    build_session(Arc::new(state))
}
