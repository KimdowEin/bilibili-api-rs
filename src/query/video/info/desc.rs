//! 视频简介

use super::VideoQuery;

/// 获取视频简介
pub const VIDEO_DESC_URL: &str = "https://api.bilibili.com/x/web-interface/archive/desc";

/// 获取视频简介
pub type VideoDescQuery = VideoQuery;

#[cfg(test)]
mod tests {
    use bili_core::Query;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[test]
    fn test_query_video_desc() {
        let query = VideoDescQuery::from(BVID);

        let url = format!("{}?{}", VIDEO_DESC_URL, query.to_query().unwrap());

        assert_eq!(
            url,
            "https://api.bilibili.com/x/web-interface/archive/desc?bvid=BV1SWfwY3ENK"
        )
    }
}
