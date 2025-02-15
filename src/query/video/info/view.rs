//! 视频基本信息

use super::VideoQuery;

/// 获取视频概览
pub const VIDEO_VIEW_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";
pub type VideoViewQuery = VideoQuery;

/// 视频超详细信息
pub const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view/detail";
pub type VideoInfoQuery = VideoQuery;

