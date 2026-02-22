//! 视频流信息

use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use typed_builder::TypedBuilder;

use crate::{
    VideoQuery,
    format::{AudioQn, Fnval, SupportFormats, VideoCodeCid, VideoQn},
};

/// 获取视频流地址
pub const VIDEO_STREAM_URL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

/// 获取视频流地址
///
/// fnval为DASH时,qn无效
///
/// sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, TypedBuilder)]
pub struct VideoStreamQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
    #[builder(default, setter(into, strip_option))]
    pub qn: Option<VideoQn>,
    #[builder(default, setter(into, strip_option))]
    pub fnval: Option<Fnval>,

    #[builder(default,setter(transform = |fourk:bool|  fourk.then_some(1).or(Some(0))))]
    pub fourk: Option<u8>,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    #[builder(default, setter(into, strip_option))]
    pub platform: Option<String>,
    // high_quality: Option<u8>,
}

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
    #[serde(rename = "video")]
    pub videos: Vec<Video>,
    #[serde(rename = "audio")]
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub audios: Vec<Audio>,
    pub dolby: Dolby,
    pub flac: Option<Flac>,
}
impl Dash {
    /// 提取对应qn的流信息(video,audio,flac_audio)
    ///
    /// 视频流同一清晰度有多个码流,谁顺序在前返回谁
    pub fn get(
        &self,
        qn: &VideoQn,
        audio_qn: &AudioQn,
    ) -> (Option<&Video>, Option<&Audio>, Option<&Audio>) {
        let video = self.videos.iter().find(|video| &video.id == qn);
        let audio = self.audios.iter().find(|audio| &audio.id == audio_qn);
        let flac = self.flac.as_ref().map(|flac| &flac.audio);
        (video, audio, flac)
    }

    pub fn get_best(&self) -> (Option<&Video>, Option<&Audio>) {
        let video = self.videos.iter().max_by_key(|video| video.id.clone());
        let audio = self.audios.iter().max_by_key(|audio| audio.id.clone());
        let flac = self.flac.as_ref().map(|flac| &flac.audio);

        let best = flac
            .is_some()
            .then_some((video, flac))
            .unwrap_or((video, audio));

        best
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: VideoQn,
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

// #[deprecated(since = "1.0.0", note = "mp4是旧格式,b站计划废弃,切换到dash")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoStreamOld {
    pub quality: VideoQn,
    pub timelength: u64,
    #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
    pub accept_format: Vec<String>,
    pub accept_description: Vec<String>,
    pub accept_quality: Vec<VideoQn>,
    pub video_codecid: VideoCodeCid,
    /// 视频流信息
    pub durl: Vec<Durl>,
    pub support_formats: Vec<SupportFormats>,
    pub last_play_time: u64,
    pub last_play_cid: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct VideoStream {
    pub quality: VideoQn,
    pub timelength: u64,
    #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
    pub accept_format: Vec<String>,
    pub accept_description: Vec<String>,
    pub accept_quality: Vec<VideoQn>,
    pub video_codecid: VideoCodeCid,
    /// 视频流信息
    pub dash: Dash,
    pub support_formats: Vec<SupportFormats>,
    pub last_play_time: u64,
    pub last_play_cid: u64,
}
