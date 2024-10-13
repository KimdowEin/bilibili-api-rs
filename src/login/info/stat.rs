use serde::{Deserialize, Serialize};

use crate::common::Query;

pub const STAT_URL: &str = "https://api.bilibili.com/x/web-interface/nav/stat";

#[derive(Debug, Serialize, Deserialize)]
pub struct StatQuery{
    access_key:String
}
impl Query for StatQuery{}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatData{
    pub following:u64,
    pub follower:u64,
    pub dynamic_count:u64,
}

