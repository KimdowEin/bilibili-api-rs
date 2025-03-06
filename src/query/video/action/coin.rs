use super::VideoQuery;
use crate::traits::Query;
use macros::Query;
use serde::{Deserialize, Serialize};

pub const COIN_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/coin/add";

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize, Query)]
pub struct CoinVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub multiply: u8,
    pub select_like: u8,
    pub csrf: String,
}
impl CoinVideoQuery {
    pub fn new(vid: VideoQuery, multiply: u8, select_like: bool, csrf: String) -> Self {
        let select_like = if select_like { 1 } else { 0 };
        Self {
            vid,
            multiply,
            select_like,
            csrf,
        }
    }
}

pub const IS_COIN_URL: &str = "https://api.bilibili.com/x/web-interface/archive/coins";

pub type IsCoinQuery = VideoQuery;
