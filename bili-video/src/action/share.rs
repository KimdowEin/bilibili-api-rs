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
    use std::sync::Arc;

    use bili_core::{BiliResponse, ToQuery};
    use bili_service::{Session, SessionState};
    use reqwest::ClientBuilder;
    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1raFvzEEuU";

    #[test]
    fn test_query_share_video() {
        let url = ShareVideoQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(SHARE_VIDEO_URL);

        assert_eq!(
            url,
            "https://api.bilibili.com/x/web-interface/share/add?bvid=BV1uSfLB3E21"
        )
    }
    #[test]
    fn test_deserialize_share_video() {
        let json = include_str!("../../../tests/datas/share_video.json");
        serde_json::from_str::<BiliResponse<ShareVideo>>(json).unwrap();
    }

    #[tokio::test]
    async fn test_get_share_video() {
        let state = SessionState::from_path("../cookies_v2.json")
            .map(Arc::new)
            .unwrap();
        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .build()
            .unwrap();
        let session = Session::new(client, state);
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

        fs::write("../tests/datas/share_video.json", &json)
            .await
            .unwrap();

        let share = serde_json::from_str::<BiliResponse<ShareVideo>>(&json)
            .unwrap()
            .data()
            .unwrap();

        assert_ne!(*share, 0);
    }
}
