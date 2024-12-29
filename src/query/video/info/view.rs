//! 视频基本信息
use serde::{Deserialize, Serialize};

use crate::traits::{Query, WbiSign};

// stat::StatData

/// 获取视频概览
pub const VIDEO_VIEW_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";
/// 视频超详细信息
pub const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view/detail";

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct VideoInfoQuery {
    aid: Option<u64>,
    bvid: Option<String>,
}
impl VideoInfoQuery {
    pub fn new<N, S>(aid: N, bvid: S) -> Self
    where
        N: Into<Option<u64>>,
        S: Into<Option<String>>,
    {
        Self {
            aid: aid.into(),
            bvid: bvid.into(),
        }
    }
}
impl Query for VideoInfoQuery {}
impl WbiSign for VideoInfoQuery {}


