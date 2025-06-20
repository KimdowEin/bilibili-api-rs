//! 视频分P

use super::VideoQuery;

///查询视频分P列表 (avid/bvid转cid)
pub const VIDEO_CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

///查询视频分P列表 (avid/bvid转cid)
pub type VideoCidsQuery = VideoQuery;

#[cfg(test)]
mod tests {
    use crate::traits::Query;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    #[test]
    fn test_query_video_cids() {
        let query = VideoCidsQuery::from(BVID);

        let url = format!("{}?{}", VIDEO_CIDS_URL, query.to_query().unwrap());

        assert_eq!(
            url,
            "https://api.bilibili.com/x/player/pagelist?bvid=BV1SWfwY3ENK"
        )
    }
}
