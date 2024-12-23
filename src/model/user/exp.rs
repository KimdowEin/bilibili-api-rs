use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LevelInfo {
    /// 当前等级
    pub current_level: u8,
    /// 当前等级经验最低值
    pub current_min: u64,
    /// 当前经验
    pub current_exp: u64,
    /// 下级等级经验
    pub next_exp: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MasterLevel {
    master_level: LiveLevel,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LiveLevel {
    level: u64,
    color: u64,
    current: (u64, u64),
    next: (u64, u64),
}
