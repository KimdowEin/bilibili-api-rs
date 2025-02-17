use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_default_from_empty_object, deserialize_vec_from_string_or_vec,
};

use super::format::{AudioQn, Qn, SupportFormats, VideoCodeCid};

/// Mp4格式视频流信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Durl {
    pub length: u64,
    pub size: u64,
    /// 有效时间120min
    pub url: String,
    /// 备用视频流
    pub backup_url: Vec<String>,
}

/// Dash格式视频流信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dash {
    /// 视频长度
    pub duration: u64,
    /// 视频流信息
    pub video: Vec<Video>,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub audio: Vec<Audio>,
    pub dolby: Dolby,
    pub flac: Option<Flac>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: Qn,
    /// 有效时间为 120min
    pub base_url: String,
    pub backup_url: Vec<String>,
    pub bandwidth: u64,
    pub mime_type: String,
    pub codecs: String,
    pub sar: String,
    pub start_with_sap: i64,
    pub segment_base: SegmentBase,
    pub codecid: VideoCodeCid,
    pub width: i64,
    pub height: i64,
    pub frame_rate: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    pub id: AudioQn,
    pub base_url: String,
    pub backup_url: Vec<String>,
    pub bandwidth: u64,
    pub mime_type: String,
    pub codecs: String,
    pub sar: String,
    pub start_with_sap: i64,
    pub segment_base: SegmentBase,
    pub codecid: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentBase {
    pub initialization: String,
    pub index_range: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dolby {
    #[serde(rename = "type")]
    pub dolby_type: u8,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub audio: Vec<Audio>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flac {
    pub display: bool,
    pub audio: Audio,
}

#[deprecated(since = "1.0.0", note = "mp4是旧格式,b站计划废弃,切换到dash")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoStreamOld {
    pub quality: Qn,
    pub timelength: u64,
    #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
    pub accept_format: Vec<String>,
    pub accept_description: Vec<String>,
    pub accept_quality: Vec<Qn>,
    pub video_codecid: VideoCodeCid,
    /// 视频流信息
    pub durl: Vec<Durl>,
    pub support_formats: Vec<SupportFormats>,
    pub last_play_time: u64,
    pub last_play_cid: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoStream {
    pub quality: Qn,
    pub timelength: u64,
    #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
    pub accept_format: Vec<String>,
    pub accept_description: Vec<String>,
    pub accept_quality: Vec<Qn>,
    pub video_codecid: VideoCodeCid,
    /// 视频流信息
    pub dash: Dash,
    pub support_formats: Vec<SupportFormats>,
    pub last_play_time: u64,
    pub last_play_cid: u64,
}
