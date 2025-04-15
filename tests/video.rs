use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestItems{
    pub bvid:Vec<String>
}

// 同步,减少压力
#[cfg(feature = "session")]
#[tokio::test]
async fn video_test() {
    use bilibili_api_rs::{query::video::info::view::VideoViewQuery, service::{video::get_video_view, Session}};
    let session = Session::new_with_path("./cookies.json").unwrap();

    let s = include_str!("./tests.toml");
    let items = toml::from_str::<TestItems>(s).unwrap();

    let bvids = items.bvid.clone();
    // 测试视频信息
    for bvid in bvids {
        let query = VideoViewQuery::from(bvid.as_str());
        let result = get_video_view(&session, query).await;

        if result.is_err() {
            println!("error bvid:{}",bvid);
        }
    }
}
