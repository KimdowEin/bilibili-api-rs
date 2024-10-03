#![allow(dead_code)]

use crate::sign::wbi::Wbi;
use serde::{Deserialize, Serialize};
#[cfg(feature = "session")]
mod session_use{
    pub use crate::session::Session;
    pub use crate::session::{Data, ResponseData};
}
#[cfg(feature = "session")]
use session_use::*;



pub const NAV_URL: &str = "https://api.bilibili.com/x/web-interface/nav";
//导航栏用户信息

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NavData {
    #[serde(rename = "isLogin")]
    is_login: bool,
    pub wbi_img: Wbi,
}
impl NavData {
    pub fn is_login(&self) -> bool {
        self.is_login
    }
}


#[cfg(feature = "session")]
impl Session {
    pub async fn nav(&self) -> Result<NavData, reqwest::Error> {
        let url = NAV_URL;
        let response = self
            .get(url)
            .send()
            .await?
            .json::<ResponseData>()
            .await?
            .take();
        if let Some(Data::NavData(data)) = response {
            Ok(data)
        } else {
            panic!("Unexpected response type")
        }
    }
}
