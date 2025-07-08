//! 获取流信息
//!
//! ```rust
//! #[tokio::test]
//! #[ignore]
//! async fn test_download_video(){
//!     const BVID:&str = "BV1RHMgz4EnY";
//!
//!     let session = Session::new_with_path("./cookies.json").unwrap();
//!
//!     let query = VideoCidsQuery::from(BVID);
//!     let cids = VideoCidsRequest::send_request(&session, query).await.unwrap();
//!     let cid = cids[0].cid;
//!
//!     let vid = VideoQuery::from(BVID);
//!     let query = VideoStreamQuery::builder()
//!        .vid(vid)
//!        .cid(cid)
//!        .fnval(Fnval::DASH)
//!        .build();
//!     let video_stream = VideoStreamRequest::send_request(&session, query).await.unwrap();
//!     let (video,audio) = video_stream.dash.get_best();
//!     if let (Some(video),Some(audio)) = (video,audio) {
//!         let temp_dir = TempDir::new().unwrap();
//!         
//!         let video_path = temp_dir.path().join("video");
//!         let audio_path = temp_dir.path().join("audio");
//!
//!         let download_video = download_file(&session, &video.base_url, &video_path);
//!         let download_audio = download_file(&session, &audio.base_url, &audio_path);
//!
//!         tokio::try_join!(download_video, download_audio).expect("下载失败");
//!
//!         let output_path = "./tests/output/output.mp4";
//!         merge_video_audio(&video_path, &audio_path, output_path).await.expect("合并失败");
//!
//!         // 删除源文件（临时文件会随 temp_dir 被自动删除）
//!         println!("合并完成，输出文件：{}", output_path);
//!     }
//!     
//! }
//!
//! async fn download_file(session: &Session, url: &str, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let response = session.get(url).send().await?;
//!     let total_size = response.content_length().unwrap_or(0);
//!
//!     let pb = ProgressBar::new(total_size as u64);
//!     pb.set_style(ProgressStyle::default_bar()
//!         .template("[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})")
//!         .unwrap()
//!         .progress_chars("#>-"));
//!
//!     let mut file = File::create(path).await?;
//!     let mut downloaded = 0u64;
//!     let mut stream = response.bytes_stream();
//!
//!     while let Some(chunk) = stream.next().await {
//!         let chunk = chunk?;
//!         file.write_all(&chunk).await?;
//!         downloaded += chunk.len() as u64;
//!         pb.set_position(downloaded);
//!     }
//!
//!     pb.finish_with_message("下载完成");
//!     Ok(())
//! }
//!
//! async fn merge_video_audio(video_path: &Path, audio_path: &Path, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let output = tokio::process::Command::new("ffmpeg")
//!         .arg("-i")
//!         .arg(video_path)
//!         .arg("-i")
//!         .arg(audio_path)
//!         .arg("-c:v")
//!         .arg("copy")
//!         .arg("-c:a")
//!         .arg("aac")
//!         .arg("-strict")
//!         .arg("experimental")
//!         .arg(output_path)
//!         .output()
//!         .await?;
//!
//!     if !output.status.success() {
//!         return Err(format!("FFmpeg 合并失败: {}", String::from_utf8_lossy(&output.stderr)).into());
//!     }
//!
//!     Ok(())
//! }
//!
//! ```

use_bili_request!();

use async_trait::async_trait;

use crate::{
    auth::{csrf, sign, to_query, AuthType},
    error::Error,
    model::video::stream::view::{VideoStream, VideoStreamOld},
    query::video::stream::{VideoStreamQuery, VIDEO_STREAM_URL},
    service::{bili_request, Session},
    traits::QueryTag,
    use_bili_request,
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
        let query = VideoStreamQuery::builder()
            .vid(query)
            .cid(cid)
            .qn(Qn::FHD)
            .fnval(Fnval::DASH | Fnval::HDR)
            .build();
        let stream = VideoStreamRequest::send_request(&session, query)
            .await
            .unwrap();
        let dash = stream.dash;
        let video1 = dash.video[0].clone();
        let url1 = video1.base_url;
        assert!(!url1.is_empty());
    }
}
