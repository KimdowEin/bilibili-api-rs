use macros::Query;
use serde::{Deserialize, Serialize};

use super::VideoQuery;
use crate::traits::Query;

// Web端点赞接口
pub const LIKE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

#[derive(Debug, PartialEq, Serialize, Deserialize, Query)]
pub struct LikeVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub like: u8,
    /// CSRF Token（位于 Cookie）
    pub csrf: String,
}

impl LikeVideoQuery {
    pub fn new(vid: VideoQuery, like: bool, csrf: String) -> Self {
        let like = if like { 1 } else { 2 };
        Self { vid, like, csrf }
    }
}
