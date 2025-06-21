//! 获取流信息

use_bili_request!();

use crate::{
    define_bili_request, model::{
        response::BiliResponse,
        video::stream::view::{VideoStream, VideoStreamOld},
    }, query::video::stream::{VideoStreamQuery, VIDEO_STREAM_URL}, traits::Sign, use_bili_request
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

define_bili_request!(VideoStream, VIDEO_STREAM_URL, Get, Sign);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        model::video::stream::format::{Fnval, Qn},
        query::video::VideoQuery,
        service::video::VideoCidsRequest,
        traits::BiliRequest,
    };

    const BVID: &str = "BV1wDCwYfE2f";

    #[tokio::test]
    async fn test_get_video_stream() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoQuery::from(BVID);
        let cid = VideoCidsRequest::send_request(&session, query.clone())
            .await
            .unwrap()[0]
            .cid;
        let query = VideoStreamQuery::new(
            query,
            cid,
            Some(Qn::FHD),
            Some(Fnval::DASH | Fnval::HDR),
            None,
            None,
        );
        let stream = VideoStreamRequest::send_request(&session, query).await.unwrap();
        let dash = stream.dash;
        let video1 = dash.video[0].clone();
        let url1 = video1.base_url;
        assert!(!url1.is_empty());
    }
}
