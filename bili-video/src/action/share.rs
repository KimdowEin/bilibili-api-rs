//! 分享视频

use bili_core::Data;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::VideoQuery;

/// 分享视频
pub const SHARE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/share/add";

/// 分享视频
///
/// csrf(post)
pub type ShareVideoQuery = VideoQuery;

/// 当前分享数
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, Deserialize, Serialize, Data)]
pub struct ShareVideo(u64);

#[cfg(test)]
mod tests {

    use bili_core::{BiliResponse, ToQuery};

    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1xqDkBREVy";

    #[tokio::test]
    #[ignore = "每次运行都要改变BVID，否则code不为0"]
    async fn test_query_share_video() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let url = ShareVideoQuery::from(BVID)
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(SHARE_VIDEO_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        fs::write("../tests/datas/video/share_video.json", &json)
            .await
            .unwrap();
    }

    #[test]
    #[ignore = "应对query时没有改BVID"]
    fn test_deserialize_share_video() {
        let json = include_str!("../../../tests/datas/video/share_video.json");
        serde_json::from_str::<BiliResponse<ShareVideo>>(json).unwrap();
    }
}
