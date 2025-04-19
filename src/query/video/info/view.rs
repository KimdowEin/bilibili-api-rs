//! 视频信息
//!
//! 不要用VideoInfoQuery,容易被风控
//!
//! 建议将其内容分开请求

use super::VideoQuery;

/// 获取视频概览
pub const VIDEO_VIEW_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view";
/// 获取视频概览
pub type VideoViewQuery = VideoQuery;

/// 视频超详细信息
pub const VIDEO_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/wbi/view/detail";
/// 视频超详细信息
pub type VideoInfoQuery = VideoQuery;

#[cfg(test)]
mod tests {
    use bili_core::Query;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[test]
    fn test_query_video_view() {
        let query = VideoViewQuery::from(BVID);

        let url = format!("{}?{}", VIDEO_VIEW_URL, query.to_query().unwrap());
        assert_eq!(
            url,
            "https://api.bilibili.com/x/web-interface/wbi/view?bvid=BV1SWfwY3ENK"
        )
    }
}
