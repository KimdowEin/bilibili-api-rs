use serde::{Deserialize, Serialize};

use super::video_info::Dimension;

///查询视频分P列表 (avid/bvid转cid)

pub const PLAYER_PAGELIST_URL: &str = "https://api.bilibili.com/x/player/pagelist";


#[derive(Debug, Serialize, Deserialize)] 
struct PlayerPageListData{
    cid: u64,
    page: u64,
    from: String,
    part: String,
    duration: u64,
    vid: String,
    weblink: String,
    dimension: Dimension,
    first_frame: String,
}