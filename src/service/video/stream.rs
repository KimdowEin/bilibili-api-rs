//! 获取流信息

use_bili_request!();

use async_trait::async_trait;

use crate::{
    auth::{csrf, sign, to_query, AuthType}, error::Error, model::video::stream::view::{VideoStream, VideoStreamOld}, query::video::stream::{VideoStreamQuery, VIDEO_STREAM_URL}, service::{bili_request, Session}, traits::QueryTag, use_bili_request
};

/// 获取视频流地址(旧Mp4格式)
#[deprecated(since = "1.0.0")]
pub struct VideoStreamOldRequest;
#[async_trait]
impl BiliRequest for VideoStreamOldRequest {
    type Query = VideoStreamQuery;
    type Response = VideoStreamOld;

    const METHOD: RequestMethod = RequestMethod::Get;
    const URL: &str = VIDEO_STREAM_URL;

    async fn send_request(session: &Session, query: Self::Query) -> Result<Self::Response, Error> {
        let url = match Self::Query::AUTH {
            AuthType::Query => format!("{}?{}", Self::URL, to_query(&query)?),
            AuthType::Sign => format!("{}?{}", Self::URL, sign(&query, &session.bili_jct().await)?),
            AuthType::Csrf => format!("{}?{}", Self::URL, csrf(&query, &session.bili_jct().await)?),
        };
        bili_request(session, url, Self::METHOD).await
    }
}

define_bili_request!(VideoStream, VIDEO_STREAM_URL, Get);

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
        let stream = VideoStreamRequest::send_request(&session, query)
            .await
            .unwrap();
        let dash = stream.dash;
        let video1 = dash.video[0].clone();
        let url1 = video1.base_url;
        assert!(!url1.is_empty());
    }
}
