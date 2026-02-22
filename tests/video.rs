use std::{
    process::Command,
    sync::{Arc, LazyLock},
};

use anyhow::Error;
use bili_auth::nav::{NAV_URL, Nav, NavQuery};
use bili_core::*;
use bili_service::{Session, SessionState};
use bili_video::{
    VideoQuery,
    format::Fnval,
    info::cids::{VIDEO_CIDS_URL, VideoCids, VideoCidsQuery},
    stream::{VIDEO_STREAM_URL, VideoStream, VideoStreamQuery},
};
use futures_util::TryStreamExt;
use reqwest::{
    ClientBuilder,
    header::{self, HeaderMap, HeaderValue},
};
use tempfile::TempDir;
use tokio::io::{AsyncWriteExt, BufWriter};

const BVID: &str = "BV1raFvzEEuU";
static HEADER: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut headers = header::HeaderMap::new();

    // 设置常见的 headers
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Linux; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
        ),
    );
    headers.insert(header::ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh-TW;q=0.9,zh;q=0.8,fr;q=0.7,en;q=0.6,ja;q=0.5"),
    );
    headers.insert(header::CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        header::REFERER,
        HeaderValue::from_static("https://www.bilibili.com/"),
    );
    headers
});

#[tokio::test]
async fn test_video_download() -> Result<(), Error> {
    let session = {
        let state = SessionState::from_path("./cookies.json").map(Arc::new)?;

        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .default_headers(HEADER.clone())
            .build()?;

        let session = Session::new(client, state);
        session.refresh_csrf();

        // let url = BiliTicketQuery::new()
        //     .to_query()?
        //     .with_csrf(&session.bili_jct())?
        //     .to_url(BILI_TICKET_URL);

        // let ticket = session
        //     .post(url)
        //     .send()
        //     .await?
        //     .json::<BiliResponse<BiliTicket>>()
        //     .await?
        //     .data()?;

        // session.set_ticket(&ticket.ticket)?;
        // let mixin_key = ticket.wbi.mixin_key();

        let url = NavQuery::new().to_query()?.to_url(NAV_URL);

        let nav = session
            .get(url)
            .send()
            .await?
            .json::<BiliResponse<Nav>>()
            .await
            .unwrap()
            .data()?;

        let mixin_key = nav.wbi_img.mixin_key();
        session.set_mixin_key(&mixin_key);

        session
    };

    let stream = {
        let url = VideoCidsQuery::from(BVID)
            .to_query()?
            .to_url(VIDEO_CIDS_URL);

        let cid = session
            .get(&url)
            .send()
            .await?
            .json::<BiliResponse<VideoCids>>()
            .await
            .unwrap()
            .data()?
            .get(0)
            .ok_or(Error::msg("cid"))?
            .cid;

        let url = VideoStreamQuery::builder()
            .cid(cid)
            .fnval(Fnval::DASH)
            .vid(VideoQuery::from(BVID))
            .build()
            .to_query()?
            .with_sign(&session.mixin_key())?
            .to_url(VIDEO_STREAM_URL);

        let stream = session
            .get(url)
            .send()
            .await?
            .json::<BiliResponse<VideoStream>>()
            .await?
            .data()?;

        stream
    };

    let (video, audio) = if let (Some(video), Some(audio)) = stream.dash.get_best() {
        (video, audio)
    } else {
        return Ok(());
    };

    let video_url = video.base_url.clone();
    let audio_url = audio.base_url.clone();

    // dbg!(&video_url, &audio_url);

    let dir = TempDir::new()?;

    let video = dir.path().join("video");
    let mut file = tokio::fs::File::create(&video).await.map(BufWriter::new)?;
    let mut stream = session.get(video_url).send().await?.bytes_stream();

    while let Some(bytes) = stream.try_next().await? {
        file.write_all(&bytes).await?;
    }

    file.flush().await?;

    // dbg!("video success");

    let audio = dir.path().join("audio");
    let mut file = tokio::fs::File::create(&audio).await.map(BufWriter::new)?;
    let mut stream = session.get(audio_url).send().await?.bytes_stream();

    while let Some(bytes) = stream.try_next().await? {
        file.write_all(&bytes).await?;
    }
    file.flush().await?;

    // dbg!("audio success");

    // dir.disable_cleanup(true);

    // dbg!(dir.keep());

    Command::new("ffmpeg")
        .arg("-i")
        .arg(video)
        .arg("-i")
        .arg(audio)
        .arg("-c:v ")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("./test/output.mp4")
        .spawn()?
        .wait()?;
    Ok(())
}
