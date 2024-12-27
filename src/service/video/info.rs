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
        cids::{CidsQuery, CIDS_URL},
        desc::VIDEO_DESC_URL,
        view::{VideoInfoQuery, VIDEO_INFO_URL, VIDEO_VIEW_URL},
    },
    service::session::Session,
    traits::Query,
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
    pub async fn get_video_view(&self, query: VideoInfoQuery) -> Result<VideoView, Error> {
        let url = format!("{}?{}", VIDEO_VIEW_URL, query.to_query()?);
        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    /// 视频cid列表
    pub async fn video_cids(&self, query: CidsQuery) -> Result<Cids, Error> {
        let url = format!("{}?{}", CIDS_URL, query.to_query()?);
        self.get(url)
            .send()
            .await?
            .json::<BiliResponse<_>>()
            .await?
            .data()
    }

    /// 获取视频简洁
    pub async fn get_video_desc(&self, query: VideoInfoQuery) -> Result<VideoDesc, Error> {
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

    const BVID: &str = "BV1wDCwYfE2f";
    use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_get_video_info() -> Result<(), Error> {
        let session = Session::new_with_path("./cookies.json")?;
        let query = VideoInfoQuery::new(None, Some(BVID.to_string()));
        let url = format!("{}?{}", VIDEO_INFO_URL, query.sign(&*session.mixin_key.read().await)?);
        println!("{}", url);
        let info = session.get(url).send().await?
            .text().await?;
        // println!("{}", info);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_video_view() -> Result<(), Error> {
        let session = Session::new_with_path("./cookies.json")?;
        let query = VideoInfoQuery::new(None, Some(BVID.to_string()));

        // let info = session.get_video_view(query).await?;
        let url = format!("{}?{}", VIDEO_VIEW_URL,query.to_query()?);
        let info = session.get(url).send().await?.text().await?;
        // println!("{}", info);

        let info:BiliResponse<VideoView> = serde_json::from_str(&info)?;

        // VideoView
        // println!("{}\n{}",info.title,info.owner.name);
        Ok(())

    }
}
