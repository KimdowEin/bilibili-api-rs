//! 视频超详细信息

use bili_core::ToQuery;
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 获取视频超详细信息(web端)
pub const VIDEO_DETAIL_URL: &str = "https://api.bilibili.com/x/web-interface/view/detail";

/// 获取视频超详细信息
///
/// sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct VideoDetailQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub need_elec: Option<u8>,
}
impl VideoDetailQuery {
    pub fn new(vid: VideoQuery, need_elec: Option<u8>) -> Self {
        Self { vid, need_elec }
    }
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_detail() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoDetailQuery {
            vid: BVID.into(),
            need_elec: Some(1),
        }
        .to_query()
        .unwrap()
        .to_url(VIDEO_DETAIL_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_detail.json", &json)
            .await
            .unwrap();
    }
}
