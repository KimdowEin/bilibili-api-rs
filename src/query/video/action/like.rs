use serde::{Deserialize, Serialize};

use crate::traits::Query;

// Web端点赞接口
pub const LIKE_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LikeVideoQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
    pub like: u8,
    /// CSRF Token（位于 Cookie）
    pub csrf: String,
}
impl Query for LikeVideoQuery {}

impl LikeVideoQuery {
    pub fn new(aid: Option<u64>, bvid: Option<String>, like: bool, csrf: String) -> Self {
        let like = if like { 1 } else { 2 };
        Self {
            aid,
            bvid,
            like,
            csrf,
        }
    }
}
