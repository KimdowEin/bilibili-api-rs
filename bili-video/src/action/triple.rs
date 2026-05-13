//! 一键三连

use bili_core::Data;
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 一键三连视频（web端）
pub const TRIPLE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like/triple";

/// 一键三连视频
pub type TripleVideoQuery = VideoQuery;

/// 一键三连返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct TripleVideo {
    pub like: bool,
    pub coin: bool,
    pub fav: bool,
    pub multiply: u8,
}

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    const BVID: &str = "BV1xqDkBREVy";

    #[tokio::test]
    #[ignore = "不知道为什么会返回-401，暂且搁置"]
    async fn test_query_triple_video() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let url = TripleVideoQuery::from(BVID)
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(TRIPLE_VIDEO_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        tokio::fs::write("../tests/datas/video/triple_video.json", &json)
            .await
            .unwrap();
    }
}
