//! 视频TAG

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取视频TAG信息（新）
pub const VIDEO_TAGS_URL: &str = "https://api.bilibili.com/x/web-interface/view/detail/tag";
/// 获取视频TAG信息（旧）
pub const VIDEO_TAGS_OLD_URL: &str = "https://api.bilibili.com/x/tag/archive/tags";

/// 获取视频TAG信息（新）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoTagsQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: Option<u64>,
}
impl VideoTagsQuery {
    pub fn new(vid: VideoQuery, cid: Option<u64>) -> Self {
        Self { vid, cid }
    }
}

/// 获取视频TAG信息（旧）
pub type VideoTagsOldQuery = VideoQuery;

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_tags() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoTagsQuery {
            vid: BVID.into(),
            cid: None,
        }
        .to_query()
        .unwrap()
        .to_url(VIDEO_TAGS_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_tags.json", &json)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_video_tags_old() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoTagsOldQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_TAGS_OLD_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_tags_old.json", &json)
            .await
            .unwrap();
    }
}
