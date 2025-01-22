use crate::{
    error::Error,
    model::{
        response::BiliResponse,
        video::info::{
            cids::Cids,
            desc::VideoDesc,
            view::{VideoInfo, VideoView},
        },
    },
    query::video::info::{
        cids::{VideoCidsQuery, CIDS_URL},
        desc::{VideoDescQuery, VIDEO_DESC_URL},
        view::{VideoInfoQuery, VideoViewQuery, VIDEO_INFO_URL, VIDEO_VIEW_URL},
    },
    service::session::Session,
    traits::{Query, Sign},
};

impl Session {
    /// 视频超详细信息
    pub async fn get_video_info(&self, query: VideoInfoQuery) -> Result<VideoInfo, Error> {
        let url = format!(
            "{}?{}",
            VIDEO_INFO_URL,
            query.sign(&*self.mixin_key.read().await)?
        );
        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    /// 获取视频概览
    pub async fn get_video_view(&self, query: VideoViewQuery) -> Result<VideoView, Error> {
        let url = format!("{}?{}", VIDEO_VIEW_URL, query.to_query()?);
        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    /// 视频cid列表
    pub async fn get_video_cids(&self, query: VideoCidsQuery) -> Result<Vec<Cids>, Error> {
        let url = format!("{}?{}", CIDS_URL, query.to_query()?);
        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<Vec<_>>>()
            .await?
            .data()
    }

    /// 获取视频简介
    pub async fn get_video_desc(&self, query: VideoDescQuery) -> Result<VideoDesc, Error> {
        let url = format!("{}?{}", VIDEO_DESC_URL, query.to_query()?);
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

    const BVID: &str = "BV1wDCwYfE2f";
 
    #[tokio::test]
    async fn test_get_video_info() {
        let mut session = Session::new_with_path("./cookies.json").unwrap();
        session.get_mixin_key().await.unwrap();
        let query = VideoInfoQuery::new(None, Some(BVID));

        let video_info = session.get_video_info(query).await.unwrap();

        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解",video_info.view.title);
    }
    
    #[tokio::test]
    async fn test_get_video_view(){
        let mut session = Session::new_with_path("./cookies.json").unwrap();
        session.get_mixin_key().await.unwrap();
        
        let query = VideoViewQuery::new(None, Some(BVID));
        
        let video_info = session.get_video_view(query).await.unwrap();
        
        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解",video_info.title);
    }

    #[tokio::test]
    async fn test_get_video_cids(){
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoCidsQuery::new(None,Some(BVID));

        let cids = session.get_video_cids(query).await.unwrap();
        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解",cids[0].part);
    }

    #[tokio::test]
    async fn test_get_video_desc(){
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoDescQuery::new(None,Some(BVID));

        let desc = session.get_video_desc(query).await.unwrap();

        assert!(!desc.is_empty());
    }


}
