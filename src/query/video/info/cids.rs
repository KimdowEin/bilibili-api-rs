//! 视频分P

use super::VideoQuery;

///查询视频分P列表 (avid/bvid转cid)
pub const CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

///查询视频分P列表 (avid/bvid转cid)
pub type VideoCidsQuery = VideoQuery;

#[cfg(test)]
mod tests {
    use bili_core::Query;

    use super::*;

    const BVID: &str = "BV1SWfwY3ENK";

    fn test_query_video_cids() {
        let query = VideoCidsQuery::from(BVID);

        let url = format!("{}?{}", CIDS_URL, query.to_query().unwrap());

        assert_eq!(
            url,
            "https://api.bilibili.com/x/player/pagelist?bvid=BV1SWfwY3ENK"
        )
    }
}
