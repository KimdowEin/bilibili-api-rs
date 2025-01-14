use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use crate::model::live::info::LiveRoomNews;

use super::{exp::{LevelView, MasterLevel}, official::{Official, OfficialVerify}, vip::Vip};

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

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Owner {
    /// UP mid
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mid: u64,
    /// UP昵称
    pub name: String,
    /// UP头像
    pub face: String,
}

/// 用户直播间信息
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

/// 视频用户栏信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerCard {
    pub card: CardView,
    pub space: Sapce,
    pub following: bool,
    pub archive_count: u64,
    pub article_count: u64,
    pub follower: u64,
    pub like_num: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardView {
    #[serde(flatten)]
    pub owner:Owner,
    pub sex: String,
    pub face_nft: u8,
    pub birthday: String,
    pub fans: u64,
    pub attention: u64,
    pub sign: String,
    pub level_info: LevelView,
    pub pendant: Pendant,
    pub nameplate: NamePlate,
    #[serde(rename = "Official")]
    pub official: Official,
    pub official_verify: OfficialVerify,
    pub vip: Vip,
    pub is_senior_member: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: u64,
    pub name: String,
    pub image: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NamePlate {
    pub nid: u64,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sapce {
    pub s_img: String,
    pub l_img: String,
}

#[derive(Debug, Default, Serialize, Deserialize, )]
pub struct Staff {
    #[serde(flatten)]
    pub owner: Owner,
    pub title: String, //名称
}
