
use bili_core::Data;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

///视频清晰度标识
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr,Data)]
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
    ///720p高帧率
    HDP = 74,
    ///1080p超清
    FHD = 80,
    ///1080P高码率
    FHDR = 112,
    ///1080P高帧率
    FHDP = 116,
    ///4K超清
    UHD = 120,
    ///HDR真彩
    HDR = 125,
    ///杜比视界
    Dolby = 126,
    ///8k超高清
    SUHD = 127,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fnval(u32);
bitflags! {
    impl Fnval:u32 {
    const MP4 = 1;
    const DASH = 16;
    const HDR = 64;
    const K4 = 128;
    const DolbyAudio = 256;
    const DolbyVision = 512;
    const K8 = 1024;
    const AV1 = 2048;
    }
}

///视频编码代码
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum VideoCodeCid {
    AVC = 7,
    HEVC = 12,
    AV1 = 13,

    #[serde(other)]
    Unknown,
}

///视频伴音音质代码
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum AudioQn {
    K64 = 30216,
    K132 = 30232,
    K192 = 30280,
    Dolby = 30250,
    HiRes = 30251,

    #[serde(other)]
    Unknown,
}
impl PartialOrd for AudioQn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AudioQn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 按照预定义的顺序排列
        let order = |value: &AudioQn| match value {
            AudioQn::K64 => 0,
            AudioQn::K132 => 1,
            AudioQn::K192 => 2,
            AudioQn::Dolby => 3,
            AudioQn::HiRes => 4,
            AudioQn::Unknown => 5,
        };

        order(self).cmp(&order(other))
    }
}

/// 支持格式的详细信息
#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize)]
pub struct SupportFormats {
    pub quality: Qn,
    pub format: String,
    pub new_description: String,
    pub display_desc: String,
    pub superscript: String,
    pub codecs: Vec<String>,
}
