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
    service::session::Session,
    traits::{Csrf, Query},
};

pub async fn like_video(session: &Session, query: LikeVideoQuery) -> Result<bool, Error> {
    let url = format!("{}?{}", LIKE_VIDEO_URL, query.csrf(&session.bili_jct().await)?);
    let response = session
        .post(url)
        .send()
        .await?
        .json::<BiliResponse<()>>()
        .await?;
    Ok(response.is_success())
}

pub async fn coin_video(session: &Session, query: CoinVideoQuery) -> Result<CoinVideo, Error> {
    let url = format!("{}?{}", COIN_VIDEO_URL, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn is_coin(session: &Session, query: IsCoinQuery) -> Result<IsCoin, Error> {
    let url = format!("{}?{}", IS_COIN_URL, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn collect_video(
    session: &Session,
    query: CollectVideoQuery,
) -> Result<CollectVideo, Error> {
    let url = format!("{}?{}", COLLECT_VIDEO_URL, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn is_collect(session: &Session, query: IsCollectQuery) -> Result<IsCollect, Error> {
    let url = format!("{}?{}", IS_COLLECT_URL, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn share_video(session: &Session, query: ShareVideoQuery) -> Result<ShareVideo, Error> {
    let url = format!("{}?{}", SHARE_VIDEO_URL, query.to_query()?);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}


#[cfg(test)]
mod test{
    use crate::query::video::VideoQuery;

    const BVID:&str="BV1AC98YeEM7";

    use super::*;

    #[tokio::test]
    async fn test_like_video(){
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoQuery::from(BVID);
        let query = LikeVideoQuery::new(query, true);

        like_video(&session, query).await.unwrap();
    }
}