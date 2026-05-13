//! 热门视频

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 获取当前热门视频列表
pub const POPULAR_URL: &str = "https://api.bilibili.com/x/web-interface/popular";

/// 获取当前热门视频列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PopularQuery {
    pub pn: Option<u64>,
    pub ps: Option<u64>,
}
impl PopularQuery {
    pub fn new(pn: Option<u64>, ps: Option<u64>) -> Self {
        Self { pn, ps }
    }
}

/// 每周必看全部列表
pub const POPULAR_SERIES_LIST_URL: &str =
    "https://api.bilibili.com/x/web-interface/popular/series/list";

/// 每周必看全部列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PopularSeriesListQuery;
impl PopularSeriesListQuery {
    pub fn new() -> Self {
        Self
    }
}

/// 每周必看选期详细信息
pub const POPULAR_SERIES_ONE_URL: &str =
    "https://api.bilibili.com/x/web-interface/popular/series/one";

/// 每周必看选期详细信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PopularSeriesOneQuery {
    pub number: u64,
}
impl PopularSeriesOneQuery {
    pub fn new(number: u64) -> Self {
        Self { number }
    }
}

/// 获取入站必刷视频
pub const POPULAR_PRECIOUS_URL: &str = "https://api.bilibili.com/x/web-interface/popular/precious";

/// 获取入站必刷视频
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct PopularPreciousQuery;
impl PopularPreciousQuery {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_popular() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PopularQuery {
            pn: Some(1),
            ps: Some(20),
        }
        .to_query()
        .unwrap()
        .to_url(POPULAR_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/popular.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_popular_series_list() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PopularSeriesListQuery
            .to_query()
            .unwrap()
            .to_url(POPULAR_SERIES_LIST_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/popular_series_list.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_popular_series_one() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PopularSeriesOneQuery { number: 1 }
            .to_query()
            .unwrap()
            .to_url(POPULAR_SERIES_ONE_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/popular_series_one.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_popular_precious() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = PopularQuery { pn: None, ps: None }
            .to_query()
            .unwrap()
            .to_url(POPULAR_PRECIOUS_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/popular_precious.json", &json)
            .await
            .unwrap();
    }
}
