use serde::{Deserialize, Serialize};

use crate::traits::{Query, Sign};

use crate::model::video::stream::{Fnval, Qn};

pub const VIDEO_STREAM_URL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VideoStreamQuery {
    pub avid: Option<u64>,
    pub bvid: Option<String>,
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
impl Query for VideoStreamQuery {}
impl Sign for VideoStreamQuery {}
impl VideoStreamQuery {
    pub fn new<N>(
        avid: N,
        bvid: Option<&str>,
        cid: u64,
        qn: Option<Qn>,
        fnval: Option<Fnval>,
        fourk: Option<bool>,
        platform: Option<String>,
    ) -> Self
    where
        N: Into<Option<u64>>,
    {
        let fourk = fourk.map(|b| b as u8);
        let bvid = bvid.map(|x| x.to_string());
        VideoStreamQuery {
            avid:avid.into(),
            bvid,
            cid,
            qn,
            fnval,
            fourk,
            platform,
        }
    }
}
