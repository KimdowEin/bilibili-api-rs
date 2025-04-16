use bilibili_api_rs::{
    query::video::info::{cids::VideoCidsQuery, desc::VideoDescQuery, view::VideoViewQuery},
    service::{video::{get_video_cids, get_video_desc, get_video_view}, Session},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestItems {
    pub bvid: Vec<String>,
}

// 同步,减少压力
#[cfg(feature = "session")]
#[tokio::test]
async fn video_test() {
    let session = Session::new_with_path("./cookies.json").unwrap();

    let s = include_str!("./tests.toml");
    let items = toml::from_str::<TestItems>(s).unwrap();

    let bvids = items.bvid.clone();

    // 测试视频信息
    test_video_view(&session, &bvids).await;

    // 测试视频简介
    test_video_desc(&session, &bvids).await;
    
    // 测试视频分P
    test_video_cids(&session, &bvids).await;
}

async fn test_video_view(session: &Session, bvids: &[String]) {
    // 测试视频信息
    for bvid in bvids {
        let query = VideoViewQuery::from(bvid.as_str());
        let result = get_video_view(&session, query).await;

        if result.is_err() {
            println!("video view error bvid:{}", bvid);
        }
    }
}


async fn test_video_desc(session: &Session, bvids: &[String]) {
    // 测试视频简介
    for bvid in bvids {
        let query = VideoDescQuery::from(bvid.as_str());
        let result = get_video_desc(&session, query).await;

        if result.is_err() {
            println!("video desc error bvid:{}", bvid);
        }
    }
}

async fn test_video_cids(session: &Session, bvids: &[String]){
    // 测试视频分P
    for bvid in bvids {
        let query = VideoCidsQuery::from(bvid.as_str());
        let result = get_video_cids(&session,query).await;

        if result.is_err() {
            println!("video cids error bvid:{}", bvid);
        }
    }
}