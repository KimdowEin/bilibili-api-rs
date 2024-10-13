#![allow(dead_code)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::{Query, WbiSign};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPlayUrlResponse {
    pub code: i32,
    pub message: String,
    pub data: WebPlayUrlData,
}
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WebPlayUrlCode {
    Success = 0,
    RequestError = -400,
    VideoNotFound = -404,
}
impl Display for WebPlayUrlCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebPlayUrlCode::Success => write!(f, "成功"),
            WebPlayUrlCode::RequestError => write!(f, "请求错误"),
            WebPlayUrlCode::VideoNotFound => write!(f, "视频不存在"),
        }
    }
}

///视频清晰度标识
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Qn {
    /// 240p急速
    FT = 6,
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
///视频流格式标识
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
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

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub enum VideoCodecId {
    AVC = 7,
    HEVC = 12,
    AV1 = 13,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub enum AudioCodecId {
    K64 = 30216,
    K132 = 30232,
    K192 = 30280,
    Dolby = 30250,
    HiRes = 30251,
}

pub const WEB_PLAYURL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPlayUrlQuery {
    // avid:i64,
    bvid: String,
    cid: u64,
    qn: Qn,
    fnval: Fnval,
    fourk: u8,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    // platform:String,
    // high_quality:i64,
}
impl Query for WebPlayUrlQuery {}
impl WbiSign for WebPlayUrlQuery {}
impl WebPlayUrlQuery {
    pub fn new(bvid: String, cid: u64, qn: Qn, fnval: Fnval) -> Self {
        let fourk = if fnval == Fnval::K4 { 1 } else { 0 };
        WebPlayUrlQuery {
            bvid,
            cid,
            qn,
            fnval,
            fourk,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPlayUrlData {
    quality: Qn,
    timelength: usize,
    accept_format: String,
    accept_quality: Vec<Qn>,
    video_codecid: VideoCodecId,
    pub durl: Option<Vec<Durl>>,
    pub dash: Option<Dash>,
    last_play_time: usize,
    last_play_cid: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Durl {
    pub length: usize,
    pub size: usize,
    pub url: String,
    pub backup_url: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dash {
    pub duration: usize,
    pub video: Vec<Video>,
    pub audio: Option<Vec<Audio>>,
    pub dolby: Dolby,
    pub flac: Option<Flac>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    pub id: Qn,
    pub base_url: String,
    pub backup_url: Vec<String>,
    pub bandwidth: usize,
    pub mime_type: String,
    pub codecs: String,
    pub sar: String,
    pub start_with_sap: i64,
    pub segment_base: SegmentBase,
    pub codecid: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentBase {
    initialization: String,
    index_range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    #[serde(flatten)]
    pub comment: Audio,
    pub width: i64,
    pub height: i64,
    pub frame_rate: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dolby {
    #[serde(rename = "type")]
    pub dolby_type: u8,
    pub audio: Vec<Audio>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flac {
    pub display: bool,
    pub audio: Audio,
}
// #[cfg(feature = "session")]
// mod session {
//     use super::*;
//     use crate::common::Session;
//     use crate::common::{Data, ResponseData};
//     use reqwest::Error;

//     impl Session {
//         /// 获取视频流地址_web端
//         pub async fn get_web_playurl(&self, query: String) -> Result<WebPlayUrlData, Error> {
//             let url = format!("{}?{}", WEB_PLAYURL, query);
//             let response = self
//                 .get(url)
//                 .send()
//                 .await?;
//             println!("response: {:?}",response.text().await.unwrap());
//             let url = format!("{}?{}", WEB_PLAYURL, query);
//             let response = self
//                 .get(url)
//                 .send()
//                 .await?
//                 .json::<ResponseData>()
//                 .await?
//                 .take();
//             if let Some(Data::WebPlayUrlData(playurl)) = response {
//                 Ok(playurl)
//             } else {
//                 panic!("Unexpected response type")
//             }
//         }
//     }
// }
// #[cfg(feature = "session")]
// pub use session::*;
