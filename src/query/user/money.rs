use serde::{Deserialize, Serialize};

use crate::traits::Query;

/// 获取硬币数
pub const COIN_URL: &str = "https://account.bilibili.com/site/getCoin";
/// 获取硬币数
#[derive(Debug,Default, Deserialize, Serialize)]
pub struct CoinQuery;
impl Query for CoinQuery {}


/// 查询硬币使用记录
pub const COIN_LOG_URL: &str = "https://api.bilibili.com/x/member/web/coin/log";

/// 查询硬币使用记录
#[derive(Debug,Default, Deserialize, Serialize)]
pub struct CoinLogQuery {
    pub access_key: Option<String>,
}
impl Query for CoinLogQuery {}

