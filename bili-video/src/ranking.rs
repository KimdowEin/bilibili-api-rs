//! 视频排行榜

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 获取分区视频排行榜列表
pub const RANKING_URL: &str = "https://api.bilibili.com/x/web-interface/ranking/v2";

/// 获取分区视频排行榜列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct RankingQuery {
    pub rid: Option<u64>,
    #[serde(rename = "type")]
    pub rank_type: Option<String>,
    pub web_location: Option<String>,
}
impl RankingQuery {
    pub fn new(rid: Option<u64>, rank_type: Option<String>, web_location: Option<String>) -> Self {
        Self {
            rid,
            rank_type,
            web_location,
        }
    }
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_ranking() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = RankingQuery {
            rid: Some(0),
            rank_type: Some("all".to_string()),
            web_location: None,
        }
        .to_query()
        .unwrap()
        .to_url(RANKING_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/ranking.json", &json)
            .await
            .unwrap();
    }
}
