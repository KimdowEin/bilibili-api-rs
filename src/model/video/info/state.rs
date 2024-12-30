
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
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
    pub evaluation: Option<String>,
    /// 禁止转载
    #[serde(default)]
    pub no_reprint: u8,
    /// 版权标志
    #[serde(default)]
    pub copyright: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Rights {
    /// 是否允许承包
    pub bp: u8,
    /// 是否支持充电
    pub elec: u8,
    /// 是否支持下载
    pub download: u8,
    /// 是否电影
    pub movie: u8,
    /// 是否PGC付费
    pub pay: u8,
    /// 是否有高码率
    pub hd5: u8,
    /// 是否禁止转载
    pub no_reprint: u8,
    /// 是否自动播放
    pub autoplay: u8,
    /// 是否UGC付费
    pub ugc_pay: u8,
    /// 是否合作视频
    pub is_cooperation: u8,
    /// 是否互动视频
    pub is_stein_gate: u8,
    /// 是否全景视频
    pub is_360: u8,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Dimension {
    pub width: u64,
    pub height: u64,
    /// 是否反转
    pub rotate: u8,
}

#[derive(Debug, Default, Serialize_repr, Deserialize_repr, )]
#[repr(i32)]
pub enum VideoState {
    OrangePass = 1,
    #[default]
    Open = 0,
    WaitReviewed=-1,
    Reject=-2,
    PoliceLock=-3,
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
    GarbageStation=-14,
    Distribute=-15,
    FormatError=-16,
    NotSubmit = -20,
    SubmitAgain = -30,
    TimingSubmit = -40,
    OnlySelf = -50,
    Delecte = -100,
}