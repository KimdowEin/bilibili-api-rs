use serde::{Deserialize, Serialize};

use crate::model::live::stream::{Qn, Quality};

pub const LIVE_STREAM_URL: &str = "https://api.live.bilibili.com/room/v1/Room/playUrl";

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStreamQuery {
    pub cid: u64,
    pub platform: Option<String>,
    pub qn: Option<Qn>,
    pub quality: Option<Quality>,

}