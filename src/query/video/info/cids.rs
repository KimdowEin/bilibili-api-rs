use serde::{Deserialize, Serialize};

use crate::traits::Query;

use super::view::VideoQuery;

///查询视频分P列表 (avid/bvid转cid)
pub const CIDS_URL: &str = "https://api.bilibili.com/x/player/pagelist";

pub type  CidsQuery = VideoQuery;