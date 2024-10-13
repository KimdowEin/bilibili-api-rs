use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::Query;

#[derive(Debug, Serialize, Deserialize)]    
pub struct StatResponse {
    pub code: StatResponseCode,
    pub message: String,
    pub data: StatData,
}
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum StatResponseCode {
    Success = 0,
    RequestError = -400,
    RequestForbidden = -412,
    RequestError1 = 40001,
    VideoNotFound = 40003,
}
impl Display for StatResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatResponseCode::Success => write!(f, "成功"),
            StatResponseCode::RequestError1 => write!(f, "请求错误"),
            StatResponseCode::VideoNotFound => write!(f, "视频不存在"),
            Self::RequestError => write!(f, "请求错误"),
            Self::RequestForbidden => write!(f, "请求被拦截"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]    
pub enum StatData {
    BvidStatData(BvidStatData),
    AvidStatData(AidStatData),
    
}

pub const AID_STAT_URL: &str = "https://api.bilibili.com/archive_stat/stat";

#[derive(Debug, Deserialize, Serialize)]
pub struct AidStatQuery {
    pub aid: u64,
}
impl AidStatQuery {
    pub fn new(aid: u64) -> Self {
        Self { aid }
    }
}
impl Query for AidStatQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AidStatData {
    pub aid: u64,
    pub view: String,
    pub danmaku: u64,
    pub reply: u64,
    pub favorite: u64,
    pub coin: u64,
    pub share: u64,
    pub now_rank: u64,
    pub his_rank: u64,
    pub like: u64,
    pub dislike: u64,
    pub no_reprint: u8,
    pub copyright: u8,
}

pub const STAT_URL: &str = "https://api.bilibili.com/x/web-interface/archive/stat";
#[derive(Debug, Deserialize, Serialize)]
pub struct StatQuery {
    aid: u64,
    bvid: String,
}
impl StatQuery {
    pub fn new<A, B>(aid: A, bvid: B) -> Self
    where
        A: Into<u64>,
        B: Into<String>,
    {
        Self {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl Query for StatQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct BvidStatData {
    pub aid: u64,
    pub bvid: String,
    pub view: String,
    pub danmaku: u64,
    pub reply: u64,
    pub favorite: u64,
    pub coin: u64,
    pub share: u64,
    pub now_rank: u64,
    pub his_rank: u64,
    pub like: u64,
    pub dislike: u64,
    pub no_reprint: u8,
    pub copyright: u8,
}
