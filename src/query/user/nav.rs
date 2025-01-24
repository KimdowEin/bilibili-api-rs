use serde::{Deserialize, Serialize};

use crate::traits::Query;

// 导航栏用户信息
pub const NAV_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/nav";

// 导航栏用户信息
#[derive(Debug,Default, Deserialize, Serialize)]
pub struct NavInfoQuery;
impl Query for NavInfoQuery {
}
