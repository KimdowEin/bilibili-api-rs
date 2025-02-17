//! 获取视频流地址

use crate::{model::video::format::{Fnval, Qn}, traits::{Query, Sign}};
use macros::{Query, Sign};
use serde::{Deserialize, Serialize};

use super::info::VideoQuery;

/// 获取视频流地址
pub const VIDEO_STREAM_URL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

/// 获取视频流地址
#[derive(Debug, Clone,PartialEq, Serialize, Deserialize, Query, Sign)]
pub struct VideoStreamQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub cid: u64,
    pub qn: Option<Qn>,
    pub fnval: Option<Fnval>,
    pub fourk: Option<u8>,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    pub platform: Option<String>,
    // high_quality: Option<u8>,
}
impl VideoStreamQuery {
    pub fn new(
        vid: VideoQuery,
        cid: u64,
        qn: Option<Qn>,
        fnval: Option<Fnval>,
        fourk: Option<bool>,
        platform: Option<String>,
    ) -> Self {
        let fourk = fourk.map(|b| b as u8);
        VideoStreamQuery {
            vid,
            cid,
            qn,
            fnval,
            fourk,
            platform,
        }
    }
}
