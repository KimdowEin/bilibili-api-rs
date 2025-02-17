use super::VideoQuery;

///查询视频分P列表 (avid/bvid转cid)
pub const CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

///查询视频分P列表 (avid/bvid转cid)
pub type VideoCidsQuery = VideoQuery;
