use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
#[derive(Debug, Serialize_repr, Deserialize_repr)]
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

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum VideoCodeId {
    AVC = 7,
    HEVC = 12,
    AV1 = 13,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum AudioCodecId {
    K64 = 30216,
    K132 = 30232,
    K192 = 30280,
    Dolby = 30250,
    HiRes = 30251,
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
    pub initialization: String,
    pub index_range: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStream {
    pub quality: Qn,
    pub timelength: usize,
    pub accept_format: String,
    pub accept_quality: Vec<Qn>,
    pub video_codecid: VideoCodeId,
    pub durl: Option<Vec<Durl>>,
    pub dash: Option<Dash>,
    pub last_play_time: usize,
    pub last_play_cid: i64,
}

