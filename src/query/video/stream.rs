use serde::{Deserialize, Serialize};

use crate::traits::{Query, WbiSign};

use crate::model::video::stream::{Fnval, Qn};

pub const VIDEO_STREAM_URL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VideoStreamQuery {
    avid: Option<u64>,
    bvid: Option<String>,
    cid: u64,
    qn: Option<Qn>,
    fnval: Option<Fnval>,
    fourk: Option<u8>,
    // session:String,
    // otype:String,
    // #[serde(rename="type")]
    // response_type:String,
    platform: Option<String>,
    // high_quality: Option<u8>,
}
impl Query for VideoStreamQuery {}
impl WbiSign for VideoStreamQuery {}
impl VideoStreamQuery {
    pub fn new<N, S>(
        avid: N,
        bvid: S,
        cid: u64,
        qn: Option<Qn>,
        fnval: Option<Fnval>,
        fourk: Option<bool>,
        platform: Option<String>,
    ) -> Self
    where
        N: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        let fourk = fourk.map(|b| b as u8);
        VideoStreamQuery {
            avid: avid.into(),
            bvid: bvid.into(),
            cid,
            qn,
            fnval,
            fourk,
            platform,
        }
    }
}
