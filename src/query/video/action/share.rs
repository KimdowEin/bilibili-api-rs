use crate::{query::video::VideoQuery, traits::Query};
use macros::Query;
use serde::{Deserialize, Serialize};

pub const SHARE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/share/add";

#[derive(Debug, PartialEq, Serialize, Deserialize, Query)]
pub struct ShareVideoQuery {
    #[serde(flatten)]
    pub vid: VideoQuery,
    pub csrf: Option<String>,
}
impl ShareVideoQuery {
    pub fn new(vid: VideoQuery, csrf: Option<String>) -> Self {
        Self { vid, csrf }
    }
}
