#![allow(dead_code)]

use crate::{
    session::{Data, ResponseData, Session},
    sign::wbi::Wbi,
};
use serde::{Deserialize, Serialize};

//导航栏用户信息

#[derive(Debug, Deserialize, Serialize)]
pub struct NavData {
    #[serde(rename = "isLogin")]
    is_login: bool,
    pub web_img: Wbi,
}

impl Session {
    pub async fn nav(&self) -> NavData {
        let url = "https://api.bilibili.com/x/web-interface/nav";
        let response = self
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<ResponseData>()
            .await
            .unwrap()
            .take()
            .unwrap();

        match response {
            Data::NavData(data) => data,
            _ => panic!("Unexpected response type"),
        }
    }
}
