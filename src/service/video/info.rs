use crate::{
    error::Error,
    model::{
        response::BiliResponse,
        video::info::{cids::Cids, desc::VideoDesc, view::VideoView},
    },
    query::video::info::{
        cids::{VideoCidsQuery, CIDS_URL},
        desc::{VideoDescQuery, VIDEO_DESC_URL},
        view::{VideoInfoQuery, VideoViewQuery, VIDEO_VIEW_URL},
    },
    service::session::Session,
    traits::Query,
};

// impl Session {
//     /// 视频超详细信息
//     #[deprecated]
//     pub async fn get_video_info(&self, query: VideoInfoQuery) -> Result<VideoInfo, Error> {
//         let url = format!(
//             "{}?{}",
//             VIDEO_INFO_URL,
//             query.sign(&*self.mixin_key.read().await)?
//         );
//         self.get(url)
//             .send()
//             .await?
//             .json::<BiliResponse<_>>()
//             .await?
//             .data()
//     }

// }

/// 视频概览
pub async fn get_video_view(session: &Session, query: VideoInfoQuery) -> Result<VideoView, Error> {
    let query = query.to_query()?;
    let url = format!("{}?{}", VIDEO_VIEW_URL, query);
    let response = session.get(&url).send().await?.text().await?;
    println!("{:?}", response);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn get_video_desc(session: &Session, query: VideoDescQuery) -> Result<VideoDesc, Error> {
    let query = query.to_query()?;
    let url = format!("{}?{}", VIDEO_DESC_URL, query);
    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<_>>()
        .await?
        .data()
}

pub async fn get_video_cids(session: &Session, query: VideoCidsQuery) -> Result<Vec<Cids>, Error> {
    let query = query.to_query()?;
    let url = format!("{}?{}", CIDS_URL, query);
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

    const BVID: &str = "BV1wDCwYfE2f";

    #[tokio::test]
    async fn test_get_video_view() {
        let mut session = Session::new_with_path("./cookies.json").unwrap();
        session.get_mixin_key().await.unwrap();

        let query = VideoViewQuery::from(BVID);

        let video_info = get_video_view(&session, query).await.unwrap();

        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解", video_info.title);
    }

    // #[tokio::test]
    // async fn test_get_video_info() {
    //     let mut session = Session::new_with_path("./cookies.json").unwrap();
    //     session.get_mixin_key().await.unwrap();
    //     let query = VideoInfoQuery::from(BVID);

    //     let video_info = get_video_info(&session,query).await.unwrap();

    //     assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解", video_info.view.title);
    // }

    #[tokio::test]
    async fn test_get_video_cids() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoCidsQuery::from(BVID);

        let cids = get_video_cids(&session, query).await.unwrap();
        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解", cids[0].part);
    }

    #[tokio::test]
    async fn test_get_video_desc() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoDescQuery::from(BVID);

        let desc = get_video_desc(&session, query).await.unwrap();

        assert!(!desc.is_empty());
    }
}
