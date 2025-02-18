use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub money: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinLog {
    pub list: Vec<CoinLogItem>,
    pub count: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinLogItem {
    pub time: String,
    pub delta: i64,
    pub reason: String,
}
