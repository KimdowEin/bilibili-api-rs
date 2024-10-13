use serde::{Deserialize, Serialize};

pub const COIN_URL: &str = "https://account.bilibili.com/site/getCoin";

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinData {
    pub money: Option<u64>,
}

