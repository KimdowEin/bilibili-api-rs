//! 视频推荐

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// 获取首页视频推荐列表（web端）
pub const RECOMMEND_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/index/top/feed/rcmd";

/// 获取首页视频推荐列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, TypedBuilder)]
pub struct RecommendQuery {
    #[builder(default, setter(into, strip_option))]
    pub fresh_type: Option<u8>,
    #[builder(default, setter(into, strip_option))]
    pub ps: Option<u8>,
    #[builder(default, setter(into, strip_option))]
    pub fresh_idx: Option<u32>,
    #[builder(default, setter(into, strip_option))]
    pub fresh_idx_1h: Option<u32>,
    #[builder(default, setter(into, strip_option))]
    pub fetch_row: Option<u32>,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_recommend() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = RecommendQuery {
            fresh_type: Some(4),
            ps: Some(12),
            fresh_idx: Some(1),
            fresh_idx_1h: Some(1),
            fetch_row: Some(1),
        }
        .to_query()
        .unwrap()
        .to_url(RECOMMEND_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/recommend.json", &json)
            .await
            .unwrap();
    }
}
