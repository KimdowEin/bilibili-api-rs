//! 视频基本信息
use serde::{Deserialize, Serialize};

use crate::traits::{Query, WbiSign};


#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct VideoQuery {
    aid: Option<u64>,
    bvid: Option<String>,
}
impl VideoQuery {
    pub fn new<N>(aid: N, bvid: Option<&str>) -> Self
    where
        N: Into<Option<u64>>,
    {
        let bvid = bvid.map(|x| x.to_string());
        Self {
            aid: aid.into(),
            bvid,
        }
    }
}
impl Query for VideoQuery {}
impl WbiSign for VideoQuery {}

/// 获取视频概览
pub const VIDEO_VIEW_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";
pub type VideoViewQuery = VideoQuery;

/// 视频超详细信息
pub const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view/detail";
pub type VideoInfoQuery = VideoQuery;
