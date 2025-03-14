use super::VideoQuery;
use crate::traits::{Query,Csrf};
use macros::{Csrf, Query};
use serde::{Deserialize, Serialize};

pub const COIN_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/coin/add";

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize, Query,Csrf)]
pub struct CoinVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    /// 投币数量
    pub multiply: u8,
    /// 附带点赞
    pub select_like: u8,
}
impl CoinVideoQuery {
    pub fn new(vid: VideoQuery, coin_two: bool, then_like: bool) -> Self {
        let select_like = if then_like { 1 } else { 0 };
        let multiply = if coin_two { 2 } else { 1 };
        Self {
            vid,
            multiply,
            select_like,
        }
    }
}

pub const IS_COIN_URL: &str = "https://api.bilibili.com/x/web-interface/archive/coins";

pub type IsCoinQuery = VideoQuery;
