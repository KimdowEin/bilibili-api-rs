//! 视频交互

use crate::{
    error::Error,
    model::{
        response::BiliResponse,
        video::action::{
            coin::{CoinVideo, IsCoin},
            collect::{CollectVideo, IsCollect},
            share::ShareVideo,
        },
    },
    query::video::action::{
        coin::{CoinVideoQuery, IsCoinQuery, COIN_VIDEO_URL, IS_COIN_URL},
        collect::{CollectVideoQuery, IsCollectQuery, COLLECT_VIDEO_URL, IS_COLLECT_URL},
        like::{LikeVideoQuery, LIKE_VIDEO_URL},
        share::{ShareVideoQuery, SHARE_VIDEO_URL},
    },
    service::{bili_query_get, session::Session},
    traits::Csrf,
};

pub async fn like_video(session: &Session, query: LikeVideoQuery) -> Result<bool, Error> {
    let url = format!(
        "{}?{}",
        LIKE_VIDEO_URL,
        query.csrf(&session.bili_jct().await)?
    );
    let response = session
        .post(url)
        .send()
        .await?
        .json::<BiliResponse<()>>()
        .await?;

    if response.is_success() {
        Ok(true)
    } else {
        Err(Error::QueryError(response.message))
    }
}

pub async fn coin_video(session: &Session, query: CoinVideoQuery) -> Result<CoinVideo, Error> {
    let url = format!(
        "{}?{}",
        COIN_VIDEO_URL,
        query.csrf(&session.bili_jct().await)?
    );
    session
        .post(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn is_coin(session: &Session, query: IsCoinQuery) -> Result<IsCoin, Error> {
    bili_query_get(session, IS_COIN_URL, query).await
}

pub async fn collect_video(
    session: &Session,
    query: CollectVideoQuery,
) -> Result<CollectVideo, Error> {
    let url = format!(
        "{}?{}",
        COLLECT_VIDEO_URL,
        query.csrf(&session.bili_jct().await)?
    );
    session
    .post(url)
    .send()
    .await?
    .json::<BiliResponse<_>>()
    .await?
    .data()
}

pub async fn is_collect(session: &Session, query: IsCollectQuery) -> Result<IsCollect, Error> {
    bili_query_get(session, IS_COLLECT_URL, query).await
}

pub async fn share_video(session: &Session, query: ShareVideoQuery) -> Result<ShareVideo, Error> {
    let url = format!(
        "{}?{}",
        SHARE_VIDEO_URL,
        query.csrf(&session.bili_jct().await)?
    );
    session
        .post(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

#[cfg(test)]
mod test {
    use crate::query::video::VideoQuery;

    const BVID: &str = "BV1biAZeLECK";

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn action_like_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoQuery::from(BVID);
        let query = LikeVideoQuery::new(query, true);

        like_video(&session, query).await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn action_coin_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        session.refresh_sign().await.unwrap();
        let vid = VideoQuery::from(BVID);

        let query = CoinVideoQuery::new(vid, true, false);
        coin_video(&session, query).await.unwrap();
    }

    #[tokio::test]
    async fn action_is_coin() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = IsCoinQuery::from(BVID);
        is_coin(&session, query).await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn action_collect_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();

        let query = CollectVideoQuery::new(114041867536793, Some(vec![137762769]), None);

        collect_video(&session, query).await.unwrap();
    }

    #[tokio::test]
    async fn action_is_collect() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = IsCollectQuery::new(114041867536793);
        is_collect(&session, query).await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn action_share_video() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = ShareVideoQuery::from(BVID);

        share_video(&session, query).await.unwrap();
    }
}
