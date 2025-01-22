use serde::{Deserialize, Serialize};

use crate::traits::Query;

// Web端点赞接口
pub const LIKE_URL: &str = "https://api.bilibili.com/x/web-interface/archive/like";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LikeVideoQuery {
    aid: Option<u64>,
    bvid: Option<String>,
    like: u8,
    /// CSRF Token（位于 Cookie）
    csrf: String,
}
impl Query for LikeVideoQuery {}

impl LikeVideoQuery {
    fn new(aid: Option<u64>, bvid: Option<String>, like: bool, csrf: String) -> Self {
        let like = if like { 1 } else { 2 };
        Self {
            aid,
            bvid,
            like,
            csrf,
        }
    }
}
