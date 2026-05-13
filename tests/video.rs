use std::process::Command;

use anyhow::Error;
use bili_auth::nav::{NAV_URL, Nav, NavQuery};
use bili_core::*;

use bili_video::{
    VideoQuery,
    format::Fnval,
    info::cids::{VIDEO_CIDS_URL, VideoCids, VideoCidsQuery},
    stream::{VIDEO_STREAM_URL, VideoStream, VideoStreamQuery},
};
use futures_util::TryStreamExt;

use tap::Tap;
use tempfile::TempDir;
use tokio::{
    fs::create_dir_all,
    io::{AsyncWriteExt, BufWriter},
};

const BVID: &str = "BV1raFvzEEuU";

#[tokio::test]
async fn test_video_download() -> Result<(), Error> {
    let session = bili_test_utils::session_from_path("./cookies.json");
    session.refresh_csrf();

    let url = NavQuery::new().to_query()?.to_url(NAV_URL);

    session
        .get(url)
        .send()
        .await?
        .json::<BiliResponse<Nav>>()
        .await
        .unwrap()
        .data()?
        .wbi_img
        .mixin_key()
        .tap_borrow(|mixin_key| session.set_mixin_key(mixin_key));

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

    let (video, audio) = if let (Some(video), Some(audio)) = stream.dash.get_best() {
        (video, audio)
    } else {
        return Ok(());
    };

    let video_url = video.base_url.clone();
    let audio_url = audio.base_url.clone();

    let dir = TempDir::new()?;

    let video = dir.path().join("video");
    let mut file = tokio::fs::File::create(&video).await.map(BufWriter::new)?;
    let mut stream = session.get(video_url).send().await?.bytes_stream();

    while let Some(bytes) = stream.try_next().await? {
        file.write_all(&bytes).await?;
    }

    file.flush().await?;

    let audio = dir.path().join("audio");
    let mut file = tokio::fs::File::create(&audio).await.map(BufWriter::new)?;
    let mut stream = session.get(audio_url).send().await?.bytes_stream();

    while let Some(bytes) = stream.try_next().await? {
        file.write_all(&bytes).await?;
    }
    file.flush().await?;

    create_dir_all("./tests/outputs/").await?;

    Command::new("ffmpeg")
        .arg("-i")
        .arg(video)
        .arg("-i")
        .arg(audio)
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("./tests/outputs/output.mp4")
        .spawn()?
        .wait()?
        .code()
        .ok_or(Error::msg("merge error"))?;

    Ok(())
}
