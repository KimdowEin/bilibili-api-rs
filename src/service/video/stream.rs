use crate::{
    error::Error,
    model::{response::BiliResponse, video::stream::VideoStream},
    query::video::stream::{VideoStreamQuery, VIDEO_STREAM_URL},
    service::session::Session,
    traits::Sign,
};

impl Session {
    /// 获取视频流地址
    pub async fn get_video_stream(&self, query: VideoStreamQuery) -> Result<VideoStream, Error> {
        let url = format!(
            "{}?{}",
            VIDEO_STREAM_URL,
            query.sign(&self.mixin_key().await)?
        );

        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::query::video::info::cids::VideoCidsQuery;

    const BVID: &str = "BV1wDCwYfE2f";

    #[tokio::test]
    async fn test_get_video_stream() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoCidsQuery::new(None, Some(BVID));
        let cid = session.get_video_cids(query).await.unwrap()[0].cid;
        let query = VideoStreamQuery::new(None, Some(BVID), cid, None, None, None, None);
        let stream = session.get_video_stream(query).await.unwrap();

        assert!(stream.durl.is_some_and(|x| !x.is_empty()));
    }
}
