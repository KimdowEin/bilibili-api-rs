use crate::traits::Query;
use serde::{Deserialize, Serialize};

pub const VIP_STAT_URL: &str = "https://api.bilibili.com/x/vip/web/user/info";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VipStatQuery;
impl Query for VipStatQuery {}
