//! 用户状态数

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

/// 关系状态数
pub const RELATION_STAT_URL: &str = "https://api.bilibili.com/x/relation/stat";
/// UP主状态数
pub const UPSTAT_URL: &str = "https://api.bilibili.com/x/space/upstat";
/// 用户导航栏状态数
pub const NAVNUM_URL: &str = "https://api.bilibili.com/x/space/navnum";

/// 关系状态数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct RelationStatQuery {
    pub vmid: u64,
}

/// UP主状态数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct UpstatQuery {
    pub mid: u64,
}

/// 用户导航栏状态数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct NavnumQuery {
    pub mid: u64,
    pub web_location: Option<String>,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_relation_stat() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = RelationStatQuery { vmid: 296909317 }
            .to_query()
            .unwrap()
            .to_url(RELATION_STAT_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/relation_stat.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_upstat() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = UpstatQuery { mid: 296909317 }
            .to_query()
            .unwrap()
            .to_url(UPSTAT_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/upstat.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_navnum() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = NavnumQuery {
            mid: 296909317,
            web_location: None,
        }
        .to_query()
        .unwrap()
        .to_url(NAVNUM_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/navnum.json", &json)
            .await
            .unwrap();
    }
}
