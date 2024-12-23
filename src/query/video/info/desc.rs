use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 获取视频简介
pub const VIDEO_DESC_URL: &str = "https://api.bilibili.com/x/web-interface/archive/desc";

#[derive(Serialize, Deserialize, Debug,Default)]
pub struct VideoDescQuery {
    aid: Option<u64>,
    bvid: Option<String>,
}
impl Query for VideoDescQuery {
}
