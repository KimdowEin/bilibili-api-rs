use crate::traits::Query;
use serde::{Deserialize, Serialize};

/// 查询每日奖励状态
pub const EXP_REWARD_URL: &str = "https://api.bilibili.com/x/member/web/exp/reward";
/// 查询每日奖励状态
#[derive(Serialize, Deserialize, Debug,Default)]
pub struct ExpRewardQuery {
    access_key: Option<String>,
}
impl Query for ExpRewardQuery {}


/// 查询每日投币获得经验数
pub const EXP_COIN_URL: &str = "https://www.bilibili.com/plus/account/exp.php";
/// 查询每日投币获得经验数
#[derive(Serialize, Deserialize, Debug,Default)]
pub struct ExpCoinQuery;
impl Query for ExpCoinQuery {}