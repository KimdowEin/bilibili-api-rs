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
