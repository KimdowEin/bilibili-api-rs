use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_string_from_number;

use super::vip::VipType;
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpReward {
    pub login: bool,
    pub watch: bool,
    pub coins: u32,
    pub share: bool,
    pub email: bool,
    pub tel: bool,
    pub safe_question: bool,
    pub identify_card: bool,
}

pub type ExpCoin = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelView {
    pub current_level: u8,
}

#[derive(Debug,Clone,PartialEq, Deserialize, Serialize)]
pub struct LevelInfo {
    /// 当前等级
    pub current_level: u8,
    /// 当前等级经验最低值
    pub current_min: u64,
    /// 当前经验
    pub current_exp: u64,
    /// 下级等级经验,满级为'-'
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub next_exp: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MasterLevel {
    pub master_level: LiveLevel,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LiveLevel {
    pub level: u64,
    pub color: u64,
    pub current: (u64, u64),
    pub next: (u64, u64),
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LiveLevelEx {
    #[serde(flatten)]
    pub level: LiveLevel,
    pub anchor_score: u64,
    pub upgrade_score: u64,
    pub master_level_color: i64,
    pub sort: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveRoomLevel {
    pub uid: u64,
    pub cost: u64,
    pub rcost: u64,
    pub user_score: String,
    pub vip: VipType,
    pub vip_time: String,
    pub svip: VipType,
    pub svip_time: String,
    pub update_time: String,
    pub master_level: LiveLevelEx,
}

impl ExpReward {
    pub fn sum_without_coin(&self) -> u32 {
        let mut sum = 0;
        [self.login, self.watch, self.share]
            .into_iter()
            .for_each(|x| sum += x as u32 * 5);

        sum += self.coins;
        sum
    }
}
