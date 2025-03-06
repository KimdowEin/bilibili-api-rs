use macros::{Csrf, Query};
use serde::{Deserialize, Serialize};

use super::VideoQuery;
use crate::traits::{Csrf, Query};

// Web端点赞接口
pub const LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Query, Csrf)]
pub struct LikeVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub like: u8,
}

impl LikeVideoQuery {
    pub fn new(vid: VideoQuery, like: bool) -> Self {
        let like = if like { 1 } else { 2 };
        Self { vid, like }
    }
}
