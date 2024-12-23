use serde::{Deserialize, Serialize};

use crate::traits::Query;

///查询视频分P列表 (avid/bvid转cid)
pub const CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct CidsQuery {
    pub aid: Option<u64>,
    pub bvid: Option<String>,
}
impl Query for CidsQuery{
}
