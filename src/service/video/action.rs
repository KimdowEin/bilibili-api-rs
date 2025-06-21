//! 视频交互

use async_trait::async_trait;

use crate::{
    define_bili_request,
    model::{
        response::BiliResponse,
        video::action::{
            coin::{CoinVideo, IsCoin},
            collect::{CollectVideo, IsCollect},
            like::LikeVideo,
            share::ShareVideo,
        },
    },
    query::video::action::{
        coin::{CoinVideoQuery, IsCoinQuery, COIN_VIDEO_URL, IS_COIN_URL},
        collect::{CollectVideoQuery, IsCollectQuery, COLLECT_VIDEO_URL, IS_COLLECT_URL},
        like::{LikeVideoQuery, LIKE_VIDEO_URL},
        share::{ShareVideoQuery, SHARE_VIDEO_URL},
    },
    use_bili_request,
};

use_bili_request!();

pub struct LikeVideoRequest;
#[async_trait]
impl BiliRequest for LikeVideoRequest {
    type Query = LikeVideoQuery;
    type Response = LikeVideo;
    const AUTH: AuthType = AuthType::Csrf;
    const METHOD: RequestMethod = RequestMethod::Post;
    const URL: &str = LIKE_VIDEO_URL;

    async fn send_request(session: &Session, query: Self::Query) -> Result<Self::Response, Error> {
        let url = format!("{}?{}", Self::URL, csrf(&query, &session.bili_jct().await)?);
        let response = session
            .post(url)
            .send()
            .await?
            .json::<BiliResponse<()>>()
            .await?;

        if response.is_success() {
            Ok(LikeVideo(true))
        } else {
            Err(Error::ResponseError {
                code: response.code,
                message: response.message,
            })
        }
    }
}

define_bili_request!(CoinVideo, COIN_VIDEO_URL, Post, Csrf);
define_bili_request!(IsCoin, IS_COIN_URL, Get, None);
define_bili_request!(CollectVideo, COLLECT_VIDEO_URL, Post, Csrf);
define_bili_request!(IsCollect, IS_COLLECT_URL, Get, None);
define_bili_request!(ShareVideo, SHARE_VIDEO_URL, Post, Csrf);

#[cfg(test)]
mod test {
    use crate::query::video::VideoQuery;

    const BVID: &str = "BV1cwKAz3EmJ";

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn action_like_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();

        let query = VideoQuery::from(BVID);
        let query = LikeVideoQuery::new(query, false);
        LikeVideoRequest::send_request(&session, query)
        .await
        .unwrap();
    
    }

    #[ignore]
    #[tokio::test]
    async fn action_coin_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        session.refresh_sign().await.unwrap();
        let vid = VideoQuery::from(BVID);

        let query = CoinVideoQuery::new(vid, false, false);
        CoinVideoRequest::send_request(&session, query)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn action_is_coin() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = IsCoinQuery::from(BVID);
        IsCoinRequest::send_request(&session, query).await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn action_collect_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();

        let query = CollectVideoQuery::new(114041867536793, Some(vec![137762769]), None);

        CollectVideoRequest::send_request(&session, query)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn action_is_collect() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = IsCollectQuery::new(114041867536793);
        IsCollectRequest::send_request(&session, query)
            .await
            .unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn action_share_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = ShareVideoQuery::from(BVID);

        ShareVideoRequest::send_request(&session, query)
            .await
            .unwrap();
    }
}
