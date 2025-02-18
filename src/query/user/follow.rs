use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 获得关注数，粉丝数，动态数
pub const D_FOLLOW_STAT_URL: &str = "https://api.bilibili.com/x/web-interface/nav/stat";
/// 获得关注数，粉丝数，动态数
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FollowStatQuery {
    pub access_key: Option<String>,
}
impl Query for FollowStatQuery {}

impl FollowStatQuery {
    pub fn new(access_key: Option<String>) -> Self {
        Self { access_key }
    }
}
