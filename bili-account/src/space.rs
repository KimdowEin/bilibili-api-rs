use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{certification::Official, pendant::Pendant, rank::AccountPowerRank, vip::Vip};

/// 用户空间详细信息
pub const ACCOUNT_SPACE_INFO_URL: &str = "https://api.bilibili.com/x/space/wbi/acc/info";

/// sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct AccountSpaceInfoQuery {
    pub mid: u64,
}

impl AccountSpaceInfoQuery {
    pub fn new(mid: u64) -> Self {
        Self { mid }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct AccountSpaceInfo {
    pub mid: u64,
    pub name: String,
    pub sex: String,
    pub face: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub face_nft: bool,
    pub sign: String,
    pub rank: AccountPowerRank,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub level: String,
    pub jointime: u64,
    pub moral: u64,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub silence: bool,
    pub coins: u64,
    pub fans_badge: bool,
    // pub fans_medal: FansMedal,
    pub official: Official,
    pub vip: Vip,
    pub pendant: Pendant,
    // pub nameplate: NamePlate,
    // pub user_honour_info: UserHonourInfo,
    pub is_followed: bool,
    pub top_photo: String,
    // pub theme:Theme
    // pub sys_notice: Option<AccountNotice>,
    // pub live_room: LiveRoom,
    pub birthday: String,
    // pub school:School,
    // pub profession:Profession,
    // #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
    // pub tags: Vec<Tag>,
    // pub series:series,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_senior_member: bool,
    // todo
    // pub elec:Elec,
    // pub contract:contract,
    // pub name_render:name_render
}
