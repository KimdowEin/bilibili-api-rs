use bili_core::{Data, ToQuery};
use serde::{Deserialize, Serialize};

use crate::wbi::Wbi;

pub const NAV_URL: &str = "https://api.bilibili.com/x/web-interface/nav";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct NavQuery;
impl NavQuery {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct Nav {
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    pub wbi_img: Wbi,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_nav() {
        let url = NavQuery::new().to_query().unwrap().to_url(NAV_URL);
        assert_eq!(url, "https://api.bilibili.com/x/web-interface/nav")
    }
}
