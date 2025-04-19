//! 导航栏用户信息
//!
//! 这个唯一的用处就是获得wbi
//!
//! 如果想获得用户信息,应该用user/account里的接口

use bili_core::Query;
use serde::{Deserialize, Serialize};

// 导航栏用户信息
pub const NAV_INFO_URL: &str = "https://api.bilibili.com/x/web-interface/nav";

// 导航栏用户信息
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Query)]
pub struct NavInfoQuery;
impl NavInfoQuery {
    pub fn new() -> Self {
        Self
    }
}
