//! 获取视频信息

use crate::{
    define_bili_request,
    model::video::info::{cids::VideoCids, desc::VideoDesc, view::VideoView},
    query::video::info::{
        cids::{VideoCidsQuery, VIDEO_CIDS_URL},
        desc::{VideoDescQuery, VIDEO_DESC_URL},
        view::{VideoViewQuery, VIDEO_VIEW_URL},
    },
    use_bili_request,
};

use_bili_request!();

define_bili_request!(VideoView, VIDEO_VIEW_URL, Get, None);
define_bili_request!(VideoDesc, VIDEO_DESC_URL, Get, None);
define_bili_request!(VideoCids, VIDEO_CIDS_URL, Get, None);

#[cfg(test)]
mod tests {
    use crate::{query::video::info::view::VideoViewQuery, service::Session};

    use super::*;

    const BVID: &str = "BV1wDCwYfE2f";

    #[tokio::test]
    async fn test_get_video_view() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        session.refresh_sign().await.unwrap();

        let query = VideoViewQuery::from(BVID);

        let video_info = VideoViewRequest::send_request(&session, query.clone()).await.unwrap();
        get_video_view(&session, query).await.unwrap();

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

        let cids = VideoCidsRequest::send_request(&session, query).await.unwrap();
        assert_eq!("躁転彼女 / 香椎モイミ feat. 雪解", cids[0].part);
    }

    #[tokio::test]
    async fn test_get_video_desc() {
        let session = Session::new_with_path("./cookies.json").unwrap();
        let query = VideoDescQuery::from(BVID);

        let desc = VideoDescRequest::send_request(&session, query).await.unwrap();

        assert!(!desc.is_empty());
    }
}
