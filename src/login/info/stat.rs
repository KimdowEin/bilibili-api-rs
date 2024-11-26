use serde::{Deserialize, Serialize};

use crate::common::Query;

/// 用户状态数
pub const STAT_URL: &str = "https://api.bilibili.com/x/web-interface/nav/stat";

#[derive(Debug, Serialize, Deserialize)]
pub struct StatQuery {
    access_key: Option<String>,
}
impl Query for StatQuery {}
impl StatQuery {
    pub fn new(access_key: Option<String>) -> Self {
        Self { access_key }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatData {
    pub following: u64,
    pub follower: u64,
    pub dynamic_count: u64,
}

