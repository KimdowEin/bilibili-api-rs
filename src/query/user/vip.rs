use serde::{Deserialize, Serialize};
use crate::traits::Query;


pub const VIP_STAT_URL: &str = "https://api.bilibili.com/x/vip/web/user/info";

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct VipStatQuery;
impl Query for VipStatQuery {}

