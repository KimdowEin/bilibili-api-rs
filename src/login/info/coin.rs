use serde::{Deserialize, Serialize};

use crate::common::Query;

/// 获取硬币数
pub const COIN_URL: &str = "https://account.bilibili.com/site/getCoin";
#[derive(Debug, Deserialize, Serialize)]
pub struct CoinQuery;
impl Query for CoinQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinData {
    pub money: Option<u64>,
}
