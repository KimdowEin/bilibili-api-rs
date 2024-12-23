use serde::{Deserialize, Serialize};

use crate::model::live::info::LiveRoomNews;

use super::{exp::MasterLevel, official::OfficialVerify};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub mid: u64,
    pub uname: String,
    pub userid:String,
    pub sign: String,
    pub birthday: String,
    pub sex: String,
    pub nick_free:bool,
    pub rank:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo2 {
    pub uid:u64,
    pub uname:String,
    pub face:String,
    pub official_verify:OfficialVerify,
    pub gender:i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLiveInfo{
    pub info: AccountInfo2,
    pub exp:MasterLevel,
    pub follower_num:u64,
    pub room_id:u64,
    pub medal_name:String,
    pub glory_count:u64,
    pub pendant:String,
    pub link_group_num:u64,
    pub room_news:LiveRoomNews,
}


