//! 分享视频

use crate::query::video::VideoQuery;

/// 分享视频
pub const SHARE_VIDEO_URL: &str = "https://api.bilibili.com/x/web-interface/share/add";

/// 分享视频
pub type ShareVideoQuery = VideoQuery;
