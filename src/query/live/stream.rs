use serde::{Deserialize, Serialize};

use crate::{
    model::live::stream::{LiveStreamQn, Quality},
    traits::Query,
};

pub const LIVE_STREAM_URL: &str = "https://api.live.bilibili.com/room/v1/Room/playUrl";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamQuery {
    pub cid: u64,
    pub platform: Option<Platform>,
    pub qn: Option<LiveStreamQn>,
    pub quality: Option<Quality>,
}
impl Query for LiveStreamQuery {}
impl LiveStreamQuery {
    pub fn new(
        cid: u64,
        qn: Option<LiveStreamQn>,
        quality: Option<Quality>,
        platform: Option<Platform>,
    ) -> Self {
        Self {
            cid,
            qn,
            quality,
            platform,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Platform {
    #[serde(rename = "h5")]
    HLS,
    #[serde(rename = "http-flv")]
    #[default]
    FLV,
}
