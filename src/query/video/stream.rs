use serde::{Deserialize, Serialize};

use crate::traits::{Query, WbiSign};

use crate::model::video::stream::{Fnval, Qn};

pub const WEB_PLAYURL: &str = "https://api.bilibili.com/x/player/wbi/playurl";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct PlayerUrlQuery {
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
impl Query for PlayerUrlQuery {}
impl WbiSign for PlayerUrlQuery {}
impl PlayerUrlQuery {
    pub fn new(
        avid: Option<u64>,
        bvid: Option<String>,
        cid: u64,
        qn: Option<Qn>,
        fnval: Option<Fnval>,
        fourk: Option<bool>,
        platform: Option<String>,
    ) -> Self {
        let fourk = fourk.map(|b| b as u8);
        PlayerUrlQuery {
            avid,
            bvid,
            cid,
            qn,
            fnval,
            fourk,
            platform,
        }
    }
}

