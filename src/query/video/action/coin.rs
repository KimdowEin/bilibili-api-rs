use serde::{Deserialize, Serialize};

use crate::traits::Query;

pub const SEND_COIN_URL: &str = "https://api.bilibili.com/x/web-interface/coin/add";

#[derive(Debug,Default,Serialize,Deserialize)]
pub struct CoinVideoQuery{
    pub aid: Option<u64>,
    pub bvid: Option<String>,
    pub multiply:u8,
    pub select_like:u8,
    pub csrf:String,
}
impl Query for CoinVideoQuery{}



