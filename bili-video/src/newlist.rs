//! 最新视频

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// 获取分区最新视频列表
pub const NEWLIST_REGION_URL: &str = "https://api.bilibili.com/x/web-interface/dynamic/region";

/// 获取分区最新视频列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct NewlistRegionQuery {
    pub rid: u64,
    pub pn: Option<u64>,
    pub ps: Option<u64>,
}
impl NewlistRegionQuery {
    pub fn new(rid: u64, pn: Option<u64>, ps: Option<u64>) -> Self {
        Self { rid, pn, ps }
    }
}

/// 获取分区近期投稿列表
pub const NEWLIST_URL: &str = "https://api.bilibili.com/x/web-interface/newlist";

/// 获取分区近期投稿列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, TypedBuilder)]
pub struct NewlistQuery {
    #[builder(default, setter(into, strip_option))]
    pub rid: Option<u64>,
    #[serde(rename = "type")]
    #[builder(default, setter(into, strip_option))]
    pub list_type: Option<u8>,
    #[builder(default, setter(into, strip_option))]
    pub pn: Option<u64>,
    #[builder(default, setter(into, strip_option))]
    pub ps: Option<u64>,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_query_newlist_region() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = NewlistRegionQuery {
            rid: 21,
            pn: Some(1),
            ps: Some(14),
        }
        .to_query()
        .unwrap()
        .to_url(NEWLIST_REGION_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/newlist_region.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_newlist() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = NewlistQuery {
            rid: Some(21),
            list_type: Some(0),
            pn: Some(1),
            ps: Some(14),
        }
        .to_query()
        .unwrap()
        .to_url(NEWLIST_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/newlist.json", &json)
            .await
            .unwrap();
    }
}
