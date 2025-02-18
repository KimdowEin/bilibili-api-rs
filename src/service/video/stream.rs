#![allow(deprecated)]

use crate::{
    error::Error,
    model::{
        response::BiliResponse,
        video::{
            format::{AudioQn, Qn},
            stream::{VideoStream, VideoStreamOld},
        },
    },
    query::video::stream::{VideoStreamQuery, VIDEO_STREAM_URL},
    service::session::Session,
    traits::Sign,
};

/// 获取视频流地址(旧Mp4格式)
#[deprecated(since = "1.0.0")]
pub async fn get_video_stream_old(
    session: &Session,
    query: VideoStreamQuery,
) -> Result<VideoStreamOld, Error> {
    let url = format!(
        "{}?{}",
        VIDEO_STREAM_URL,
        query.sign(&session.mixin_key().await)?
    );

    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

/// 获取视频流地址(Dash格式)
pub async fn get_video_stream(
    session: &Session,
    query: VideoStreamQuery,
) -> Result<VideoStream, Error> {
    let url = format!(
        "{}?{}",
        VIDEO_STREAM_URL,
        query.sign(&session.mixin_key().await)?
    );

    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        model::video::format::{Fnval, Qn},
        query::video::info::VideoQuery,
        service::video::get_video_cids,
        traits::Query,
    };

    const BVID: &str = "BV1wDCwYfE2f";

    #[tokio::test]
    async fn test_get_video_stream() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoQuery::from(BVID);
        let cid = get_video_cids(&session, query.clone()).await.unwrap()[0].cid;
        let query = VideoStreamQuery::new(
            query,
            cid,
            Some(Qn::FHD),
            Some(Fnval::DASH | Fnval::HDR),
            None,
            None,
        );
        let stream = get_video_stream(&session, query).await.unwrap();
        let dash = stream.dash;
        println!("{:?}", dash);
        let video1 = dash.video[0].clone();
        let url1 = video1.base_url;
        assert!(!url1.is_empty());
    }
}
