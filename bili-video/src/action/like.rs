//! 点赞

use bili_core::{Data, ToQuery};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::VideoQuery;

// Web端点赞接口
pub const LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

/// Web端点赞接口
///
/// csrf
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct LikeVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub like: u8,
}

impl LikeVideoQuery {
    pub fn new(vid: VideoQuery, like: bool) -> Self {
        let like = like.then_some(1).unwrap_or(2);
        Self { vid, like }
    }
}

/// 点赞返回
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct LikeVideo;

pub const IS_LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/has/like";

pub type IsLikeVideoQuery = VideoQuery;

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Deserialize, Serialize, Data)]
pub struct IsLikeVideo(#[serde(deserialize_with = "deserialize_bool_from_anything")] pub bool);

#[cfg(test)]
mod tests {
    use bili_core::{BiliResponse, ToQuery};
    use tokio::fs;

    use super::*;

    const BVID: &str = "BV1xqDkBREVy";

    #[tokio::test]
    #[ignore = "每次运行都要改变BVID，否则code不为0"]
    async fn test_query_like_video() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");
        session.refresh_csrf().unwrap();

        let url = LikeVideoQuery::new(BVID.into(), true)
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(LIKE_VIDEO_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        fs::write("../tests/datas/video/like_video.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_like_video() {
        let json = include_str!("../../../tests/datas/video/like_video.json");
        serde_json::from_str::<BiliResponse<LikeVideo>>(json).unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_query_is_like_video() {
        let session = bili_test_utils::session_from_path("../cookies_v2.json");

        let url = IsLikeVideoQuery::from(BVID)
            .to_query()
            .unwrap()
            .to_url(IS_LIKE_VIDEO_URL);

        let json = session.get(url).send().await.unwrap().text().await.unwrap();

        fs::write("../tests/datas/video/is_like.json", &json)
            .await
            .unwrap();
    }

    #[test]
    fn test_deserialize_is_like_video() {
        let json = include_str!("../../../tests/datas/video/is_like.json");
        serde_json::from_str::<BiliResponse<IsLikeVideo>>(json).unwrap();
    }
}
