
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoStat {
    /// 稿件avid
    pub aid: u64,
    /// 播放数
    pub view: u64,
    /// 弹幕数
    pub danmaku: u64,
    /// 评论数
    pub reply: u64,
    /// 收藏数
    pub favorite: u64,
    /// 投币数
    pub coin: u64,
    /// 分享数
    pub share: u64,
    /// 当前排名
    pub now_rank: u64,
    /// 历史最高排名
    pub his_rank: u64,
    /// 点赞数
    pub like: u64,
    /// 点踩数
    pub dislike: u64,
    /// 视频评分
    pub evaluation: String,
    // /// 禁止转载
    // #[serde(default)]
    // pub no_reprint: u8,
    // /// 版权标志
    // #[serde(default)]
    // pub copyright: u8,
}

/// 视频属性标志
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rights {
    /// 是否允许承包
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub bp: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    /// 是否支持充电
    pub elec: bool,
    /// 是否支持下载
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub download: bool,
    /// 是否电影
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub movie: bool,
    /// 是否PGC付费
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub pay: bool,
    /// 是否有高码率
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub hd5: bool,
    /// 是否禁止转载
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub no_reprint: bool,
    /// 是否自动播放
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub autoplay: bool,
    /// 是否UGC付费
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub ugc_pay: bool,
    /// 是否UGC付费预览
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub ugc_pay_preview:bool,
    /// 是否合作视频
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_cooperation: bool,
    /// 是否互动视频
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_stein_gate: bool,
    /// 是否全景视频
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_360: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub no_background: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub clean_mode: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub no_share:bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub arc_pay:bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub free_watch:bool,
}

/// 分辨率
#[derive(Debug, Clone, PartialEq,Eq, Serialize, Deserialize)]
pub struct Dimension {
    pub width: u64,
    pub height: u64,
    /// 是否反转
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub rotate: bool,
}

#[derive(Debug, Clone, PartialEq,Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum VideoState {
    OrangePass = 1,
    Open = 0,
    WaitReviewed = -1,
    Reject = -2,
    PoliceLock = -3,
    Repeat = -4,
    Lock = -5,
    RejectReviewed = -6,
    StopReview = -7,
    AgainReviewed = -8,
    WaitFormat = -9,
    PutOffReviewed = -10,
    ResouceBrocken = -11,
    SaveError = -12,
    CommentReviewed = -13,
    GarbageStation = -14,
    Distribute = -15,
    FormatError = -16,
    NotSubmit = -20,
    SubmitAgain = -30,
    TimingSubmit = -40,
    OnlySelf = -50,
    Delecte = -100,

    #[serde(other)]
    Unknown,
}

/// 视频类型,原创/转载
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr,Eq)]
#[repr(u8)]
pub enum VideoCopyRight {
    /// 原创
    Original,
    /// 转载
    RePrint,

    #[serde(other)]
    Unknown,
}

/// 充电状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpowerState {
    /// 是否为充电专属
    pub is_upower_exclusive: bool,
    /// 未知
    pub is_upower_play: bool,
    /// 未知
    pub is_upower_preview: bool,
}