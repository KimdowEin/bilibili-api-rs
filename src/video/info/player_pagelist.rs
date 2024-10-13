use serde::{Deserialize, Serialize};

use super::view::Dimension;

///查询视频分P列表 (avid/bvid转cid)

pub const PLAYER_PAGELIST_URL: &str = "https://api.bilibili.com/x/player/pagelist";


#[derive(Debug, Serialize, Deserialize)] 
pub struct PlayerPageListData{
    pub cid: u64,
    pub page: u64,
    pub from: String,
    pub part: String,
    pub duration: u64,
    pub vid: String,
    pub weblink: String,
    pub dimension: Dimension,
    pub first_frame: String,
}