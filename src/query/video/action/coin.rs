//! 投币

use super::VideoQuery;
use crate::{Csrf, Query};
use serde::{Deserialize, Serialize};

/// 投币视频（web端）
pub const COIN_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/coin/add";

/// 投币视频（web端）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query, Csrf)]
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
        let multiply = if coin_two { 2 } else { 1 };
        let select_like = if then_like { 1 } else { 0 };
        Self {
            vid,
            multiply,
            select_like,
        }
    }
}

/// 查询是否投币
pub const IS_COIN_URL: &str = "https://api.bilibili.com/x/web-interface/archive/coins";

/// 查询是否投币
pub type IsCoinQuery = VideoQuery;
