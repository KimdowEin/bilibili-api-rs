//! 相关视频推荐

use crate::VideoQuery;

/// 获取单视频推荐列表（web端）
pub const VIDEO_RELATED_URL: &str = "https://api.bilibili.com/x/web-interface/archive/related";

/// 获取单视频推荐列表
pub type VideoRelatedQuery = VideoQuery;

#[cfg(test)]
mod tests {

    use bili_core::ToQuery;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[tokio::test]
    #[ignore]
    async fn test_query_video_related() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = VideoRelatedQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(VIDEO_RELATED_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        tokio::fs::write("../tests/datas/video/video_related.json", &json)
            .await
            .unwrap();
    }
}
