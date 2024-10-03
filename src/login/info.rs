#![allow(dead_code)]

use crate::{
    session::{Data, ResponseData, Session},
    sign::wbi::Wbi,
};
use serde::{Deserialize, Serialize};

//导航栏用户信息

#[derive(Debug, Deserialize, Serialize,Clone)]
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

impl Session {
    pub async fn nav(&self) -> Result<NavData, reqwest::Error> {
        let url = "https://api.bilibili.com/x/web-interface/nav";
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
