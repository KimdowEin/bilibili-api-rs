#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    session::{Data, Query, ResponseData, Session},
    sign::wbi::WbiSign,
};

///视频清晰度标识
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Qn {
    ///360p流畅
    FL = 16,
    ///480p清晰
    SD = 32,
    ///720p高清
    HD = 64,
    ///720p60高帧率
    HDP = 74,
    ///1080p超清
    FHD = 80,
}
impl Qn {
    fn decode(self) -> u32 {
        self as u32
    }
}
///视频流格式标识
#[derive(Debug, Serialize_repr, Deserialize_repr,PartialEq)]
#[repr(u32)]
pub enum Fnval {
    MP4 = 1,
    DASH = 16,
    HDR = 64,
    K4 = 128,
    DolbyAudio = 256,
    DolbyVision = 512,
    K8 = 1024,
    AV1 = 2048,
}
impl Fnval {
    fn decode(self) -> u32 {
        self as u32
    }
}
#[derive(Debug, Serialize_repr, Deserialize_repr,PartialEq)]
#[repr(u32)]
pub enum CodecId{
    AVC = 7,
    HEVC = 12,
    AV1 = 13,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPlayerUrlQuery {
    // avid:i64,
    pub bvid: String,
    pub cid: u64,
    qn: u32,
    fnval: u32,
    // fnver:i64,
    fourk: u8,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    // platform:String,
    // high_quality:i64,
}
impl Query for WebPlayerUrlQuery {}
impl WbiSign for WebPlayerUrlQuery {}
impl WebPlayerUrlQuery {
    fn new(bvid: String, cid: u64, qn: Qn, fnval: Fnval) -> Self {
        let fourk = if fnval == Fnval::K4 { 1 } else { 0 };
        WebPlayerUrlQuery {
            bvid,
            cid,
            qn: qn.decode(),
            fnval: fnval.decode(),
            fourk,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPlayUrlData {
    quality:Qn,
    timelength: usize,
    accept_format: String,
    accept_quality: Vec<Qn>,
    video_codecid: CodecId,
    durl: Option<Vec<Durl>>,
    dash: Option<Dash>,
    last_play_time: usize,
    last_play_cid: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Durl {
    length: usize,
    size: usize,
    url: String,
    backup_url: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dash {
    duration: usize,
    video: Vec<Video>,
    audio: Option<Vec<Audio>>,
    dolby: Dolby,
    flac: Option<Flac>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    id: Qn,
    base_url: String,
    backup_url: Vec<String>,
    bandwidth: usize,
    mime_type: String,
    codecs: String,
    sar: String,
    start_with_sap: i64,
    segment_base: SegmentBase,
    codecid: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentBase {
    initialization: String,
    index_range: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Video {
    #[serde(flatten)]
    comment: Audio,
    width: i64,
    height: i64,
    frame_rate: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Dolby {
    #[serde(rename = "type")]
    dolby_type: u8,
    audio: Vec<Audio>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Flac {
    display: bool,
    audio: Audio,
}

impl Session {
    /// 获取视频流地址_web端
    pub async fn get_web_playurl(&self, query: String) -> Result<WebPlayUrlData, reqwest::Error> {
        let url = format!(
            "{}?{}",
            "https://api.bilibili.com/x/player/wbi/playurl", query
        );
        let response = self
            .get(url)
            .send()
            .await?
            .json::<ResponseData>()
            .await?
            .take();
        if let Some(Data::WebPlayUrlData(playurl)) = response {
            Ok(playurl)
        } else {
            panic!("Unexpected response type")
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::{fs, io::AsyncWriteExt};

    use crate::video::info::WebVideoInfoQuery;

    use super::*;
    #[tokio::test]
    async fn test_get_web_playurl() {
        let mut session = Session::new();
        let mixin_key = fs::read_to_string("./test/key.txt").await.unwrap();
        session.set_mixin_key(mixin_key);
        let bvid = "BV1QVtfejExd".to_owned();
        let query = WebVideoInfoQuery::new(None, bvid.clone())
            .to_query()
            .unwrap();
        let cid = session.get_web_video_info(query).await.unwrap().cid;

        let query = WebPlayerUrlQuery::new(bvid, cid, Qn::HDP, Fnval::MP4)
            .to_query()
            .unwrap();
        let data= session.get_web_playurl(query).await.unwrap();
        let url = data.durl.unwrap()[0].url.clone();
        let mut file = fs::File::create_new("./test/video.mp4").await.unwrap();
        let mut stream = session.get(url).send().await.unwrap().bytes_stream();
        use futures_util::StreamExt;

        while let Some(item) = stream.next().await {
            file.write(&item.unwrap()).await.unwrap();
        }

    }
}
